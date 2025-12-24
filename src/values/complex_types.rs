use std::collections::HashMap;
use std::hash::Hash;

use crate::raw::{self, scm_c_vector_set_x, scm_c_make_vector};
use crate::SCM;
use crate::*;

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

impl<F:Into<SCM> + Clone> From<&[F]> for Pair {
    fn from(slice: &[F]) -> Pair {
        let mut result:Option<Pair> = None;
        for f in slice.iter().rev() {
            let scm:SCM = f.clone().into();
            result = match result {
                Some(accumulated_pair) => Some(Pair::new(scm.clone(), SCM::from(accumulated_pair))),
                None => Some(Pair::new(scm.clone().into(), SCM::eol())),
            };
        };
        result.unwrap_or(Pair::new(SCM::eol(), SCM::eol()))
    }
}

impl<F:Into<SCM> + Clone> From<Vec<F>> for Pair {
    fn from(vec: Vec<F>) -> Pair {
        Pair::from(vec.as_slice())
    }
}

impl<F:TryFrom<SCM> + Clone> TryFrom<Pair> for Vec<F> {
    type Error = String;

    fn try_from(pair: Pair) -> Result<Self, Self::Error> {
        let mut result:Vec<F> = Vec::new();
        let mut current_pair = pair;
        loop {
            let car_scm = current_pair.car;
            let elem:F = match F::try_from(car_scm) {
                Ok(val) => val,
                Err(_) => return Err("Failed to convert pair element".to_string()),
            };
            result.push(elem);
            let cdr_scm = current_pair.cdr;
            if cdr_scm.is_null() {
                break;
            } else if cdr_scm.is_pair() {
                current_pair = match Pair::try_from(cdr_scm) {
                    Ok(p) => p,
                    Err(_) => return Err("Failed to convert cdr to pair".to_string()),
                };
            } else {
                return Err("Wrong Type: cdr is neither eol nor pair.".to_string());
            }
        }
        Ok(result)
    }
}

impl<F: Into<SCM> + Clone> From<&[F]> for SCM {
    fn from(slice: &[F]) -> SCM {
        let size:usize = slice.len();
        let vector = unsafe { scm_c_make_vector(size,SCM::unspecified().0) };
        for (i, elem) in slice.iter().enumerate() {
            unsafe {
                scm_c_vector_set_x(vector, i, elem.clone().into().0);
            }
        };
        
        SCM::new(vector)
    }
}

impl<F: Into<SCM> + Clone> From<Vec<F>> for SCM {
    fn from(vec: Vec<F>) -> SCM {
        SCM::from(vec.as_slice())
    }
}

impl<F:TryFrom<SCM> + Clone> TryFrom<SCM> for Vec<F> {
    type Error = String;

    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if scm.is_vector() {
            let size = unsafe { raw::scm_c_vector_length(scm.0) };
            let mut result:Vec<F> = Vec::with_capacity(size as usize);
            for i in 0..size {
                let elem_scm = unsafe { raw::scm_c_vector_ref(scm.0, i) };
                let elem:F = match F::try_from(SCM::new(elem_scm)) {
                    Ok(val) => val,
                    Err(_) => return Err("Failed to convert vector element".to_string()),
                };
                result.push(elem);
            }
            Ok(result)
        } else if scm.is_list() {
            let pair = Pair::try_from(scm)?;
            Vec::<F>::try_from(pair)
        } else {
            Err("Wrong Type: SCM is not a vector.".to_string())
        }
    }
}

impl<F:Into<SCM>,R:Into<SCM>> From<HashMap<F,R>> for Pair {
    fn from(map: HashMap<F,R>) -> Pair {
        let scms:Vec<SCM> = map.into_iter().map(|(key, value)| {
            SCM::from(Pair::new(key.into(), value.into()))
        }).collect();
        Pair::from(scms.as_slice())
    }
}

impl<F:Into<SCM>,R:Into<SCM>> From<HashMap<F,R>> for SCM {
    fn from(map: HashMap<F,R>) -> SCM {
        let hash_table_scm = unsafe { raw::scm_c_make_hash_table(map.len() as u64) };

        for (key,value) in map.into_iter() {
            unsafe {
                raw::scm_hash_set_x(
                    hash_table_scm,
                    key.into().0,
                    value.into().0,
                );
            }
        }
        
        SCM::new(hash_table_scm)
    }
}

impl<F:TryFrom<SCM> + Eq + Hash,R:TryFrom<SCM>> TryFrom<Pair> for HashMap<F,R> {
    type Error = String;

    fn try_from(pair: Pair) -> Result<Self, Self::Error> {
        let mut result:HashMap<F,R> = HashMap::new();
        let vec = Vec::<SCM>::try_from(pair)?;
        for pair_scm in vec.into_iter() {
            let pair = Pair::try_from(pair_scm)?;
            let key_scm = pair.car;
            let value_scm = pair.cdr;
            let key:F = match F::try_from(key_scm) {
                Ok(val) => val,
                Err(_) => return Err("Failed to convert hash map key".to_string()),
            };
            let value:R = match R::try_from(value_scm) {
                Ok(val) => val,
                Err(_) => return Err("Failed to convert hash map value".to_string()),
            };
            result.insert(key, value);
        };
        Ok(result)
    }
}


impl<F:TryFrom<SCM> + Eq + Hash,R:TryFrom<SCM>> TryFrom<SCM> for HashMap<F,R> {
    type Error = String;

    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if scm.is_hash_table() {            
            let alist_scm = scm.hash_map_to_alist();
            let alist_pair = Pair::try_from(alist_scm)?;
            let result:HashMap<F,R> = HashMap::<F,R>::try_from(alist_pair)?;
            Ok(result)            
        } else if scm.is_list() {
            let pair = Pair::try_from(scm)?;
            let result:HashMap<F,R> = HashMap::<F,R>::try_from(pair)?;
            Ok(result)
        }else {
            Err("Wrong Type: SCM is not a hash table.".to_string())
        }
    }
}
