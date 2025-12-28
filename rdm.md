
# Table of Contents
1.  [Discription](#org8ed5a4a)
2.  [Overview](#org90495c9)
3.  [Project Structure](#org009c819)
4.  [Features](#org59409de)
5.  [Quick Start](#org73a6e67)
    1.  [Use Ngrs](#orga9fcd08)
    2.  [Example](#org61b62cc)
    3.  [Initialization](#orgae19dec)
        1.  [Using with \`init\`](#orge54ca79)
        2.  [Using with \`with<sub>guile</sub>\`](#org95ecb68)

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<tbody>
<tr>
<td class="org-left">English</td>
<td class="org-left"><a href="./README_CN.html">中文</a></td>
</tr>
</tbody>
</table>


<a id="org8ed5a4a"></a>

# Discription

NGRS is a New Rust bindings for GNU Guile Scheme.


<a id="org90495c9"></a>

# Overview

\`ngrs\` provides both low-level raw bindings and high-level safe abstractions for embedding GNU Guile Scheme in Rust applications. This project enables seamless integration of Scheme scripting capabilities into Rust programs with a focus on memory safety and ergonomic APIs.


<a id="org009c819"></a>

# Project Structure

-   ****ngrs**** - High-level safe Rust wrappers with idiomatic interfaces, Contains **raw**.
-   ****raw**** - Low-level FFI bindings to Guile's C API


<a id="org59409de"></a>

# Features

-    Make bindings and convert for base values.
-    Implement eval(eval<sub>expr</sub>), eval<sub>string</sub>, eval<sub>file</sub>(load), apply(apply<sub>scm</sub>), apply(call<sub>0</sub>~n), define
-    Create safe type wrappers for composite values List, Vector, HashMap and special base values procedure, Symbol, Keywords (or at least provide a method to convert them to Rust types)
-    Add module operations to facilitate writing interactive modules for Guile in Rust.
-    Write bindings to convert Rust structs into Guile foreign types


<a id="org73a6e67"></a>

# Quick Start


<a id="orga9fcd08"></a>

## Use Ngrs

Add this to your **Cargo.toml** .

    [dependencies]
    ngrs = "0.1"


<a id="org61b62cc"></a>

## Example

    use ngrs::{Runtime, with_guile};
    
    fn main() {
        // Initialize Guile
        Runtime::initialize();
    
        with_guile(|vm:Runtime| {
            // Your Code Here
            println!("Hello guile from Rust!");
            let args = vec!["Test Guile".to_string(),];
            vm.shell(args);
        });
    }


<a id="orgae19dec"></a>

## Initialization

Before using any Guile functionality, you must initialize the Guile environment:

    use ngrs::Runtime
    
    fn main() {
        // Initialize Guile
        Runtime::initialize();
    
        // Your code here
    }


<a id="orge54ca79"></a>

### Using with \`init\`

This way has Less platforms support.

    use ngrs::Runtime;
    
    fn main() {
        Runtime::initialize();
        let runtime = Runtime::new();
    
        // Your Guile-dependent code here
    }


<a id="org95ecb68"></a>

### Using with \`with<sub>guile</sub>\`

For more control over the Guile context:

    use ngrs::with_guile;
    
    fn main() {
        Runtime::initialize();
    
        with_guile(|vm:Runtime| {
            // Your Guile-dependent code here
        });
    }

