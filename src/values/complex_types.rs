use std::result;

use crate::raw;
use crate::SCM;

#[derive(Debug, Clone)]
pub struct Pair {
    pub car: SCM,
    pub cdr: SCM,
}

pub enum SCMOrPair {
    Pair(Pair),
    Other(SCM),    
}

impl SCMOrPair {

    pub fn is_pair(&self) -> bool {
        matches!(self, SCMOrPair::Pair(_))
    }

    pub fn is_scm(&self) -> bool {
        matches!(self, SCMOrPair::Other(_))
    }
    
    pub fn unwrap_pair(self) -> Pair {
        match self {
            SCMOrPair::Pair(pair) => pair,
            SCMOrPair::Other(_) => panic!("Called unwrap_pair on SCMOrPair::Other"),
        }
    }

    pub fn unwrap_scm(self) -> SCM {
        match self {
            SCMOrPair::Other(scm) => scm,
            SCMOrPair::Pair(_) => panic!("Called unwrap_scm on SCMOrPair::Pair"),
        }
    }

    pub fn unwrap_pair_or(self, default: Pair) -> Pair {
        match self {
            SCMOrPair::Pair(pair) => pair,
            SCMOrPair::Other(_) => default,
        }
    }

    pub fn unwrap_scm_or(self, default: SCM) -> SCM {
        match self {
            SCMOrPair::Other(scm) => scm,
            SCMOrPair::Pair(_) => default,
        }
    }
}

impl Pair {

    pub fn new(car: SCM,cdr: SCM) -> Self {
        Pair::cons(car,cdr)
    }
    
    pub fn cons(car: SCM, cdr: SCM) -> Self {
        Pair { car, cdr }
    }

    pub fn pretend(&self,car:SCM) -> Self {
        Pair {car, cdr: self.cdr.clone().into() }
    }

    pub fn car(&self) -> SCMOrPair {
        let car_scm = &self.car;
        car_scm.clone().into()
    }

    pub fn cdr(&self) -> SCMOrPair {
        let cdr_scm = &self.cdr;
        cdr_scm.clone().into()
    }
}

impl From<Pair> for SCM {
    fn from(pair: Pair) -> SCM {
        unsafe {
            let car_scm = pair.car.0;
            let cdr_scm = pair.cdr.0;
            let scm_pair = raw::scm_cons(car_scm, cdr_scm);
            SCM::new(scm_pair)
        }
    }
}

impl TryFrom<SCM> for Pair {
    type Error = String;

    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if scm.is_pair() {
            unsafe {
                let car_scm = raw::scm_car(scm.0);
                let cdr_scm = raw::scm_cdr(scm.0);
                Ok(Pair {
                    car: SCM::new(car_scm),
                    cdr: SCM::new(cdr_scm),
                })
            }
        } else {
            Err("Wrong Type: SCM is not a pair.".to_string())
        }
    }
}

impl From<SCM> for SCMOrPair {
    fn from(scm: SCM) -> SCMOrPair {
        if scm.is_pair() {
            match Pair::try_from(scm.clone()) {
                Ok(pair) => SCMOrPair::Pair(pair),
                Err(_) => SCMOrPair::Other(scm),
            }
        } else {
            SCMOrPair::Other(scm)
        }
    }
}

impl From<SCMOrPair> for SCM {
    fn from(scm_or_pair: SCMOrPair) -> SCM {
        match scm_or_pair {
            SCMOrPair::Pair(pair) => SCM::from(pair),
            SCMOrPair::Other(scm) => scm,
        }
    }
}

impl From<&[SCM]> for Pair {
    fn from(slice: &[SCM]) -> Pair {
        let mut result:Option<Pair> = None;
        for scm in slice.iter().rev() {
            result = match result {
                Some(accumulated_pair) => Some(Pair::new(scm.clone(), SCM::from(accumulated_pair))),
                None => Some(Pair::new(scm.clone(), SCM::eol())),
            };
        };
        result.unwrap_or(Pair::new(SCM::eol(), SCM::eol()))
    }
}
