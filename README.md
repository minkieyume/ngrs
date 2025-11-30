
# Table of Contents

1.  [Discription](#orgfb6ad3a)
2.  [Overview](#orgfa6110b)
3.  [Project Structure](#org6e78bd1)
4.  [Features](#org7f751de)
5.  [Quick Start](#org6751908)
    1.  [Use Ngrs](#org6bc8d4b)
    2.  [Example](#org1307785)
    3.  [Initialization](#orgbf47cf2)
        1.  [Using with \`init\`](#org128b3a3)
        2.  [Using with \`with<sub>guile</sub>\`](#org5b624fe)

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


<a id="orgfb6ad3a"></a>

# Discription

NGRS is a New Rust bindings for GNU Guile Scheme.


<a id="orgfa6110b"></a>

# Overview

\`ngrs\` provides both low-level raw bindings and high-level safe abstractions for embedding GNU Guile Scheme in Rust applications. This project enables seamless integration of Scheme scripting capabilities into Rust programs with a focus on memory safety and ergonomic APIs.


<a id="org6e78bd1"></a>

# Project Structure

-   ****ngrs**** - High-level safe Rust wrappers with idiomatic interfaces, Contains **raw**.
-   ****raw**** - Low-level FFI bindings to Guile's C API


<a id="org7f751de"></a>

# Features

-   Make bindings and convert for base values.


<a id="org6751908"></a>

# Quick Start


<a id="org6bc8d4b"></a>

## Use Ngrs

Add this to your **Cargo.toml** .

    [dependencies]
    ngrs = "0.1"


<a id="org1307785"></a>

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


<a id="orgbf47cf2"></a>

## Initialization

Before using any Guile functionality, you must initialize the Guile environment:

    use ngrs::Runtime
    
    fn main() {
        // Initialize Guile
        Runtime::initialize();
    
        // Your code here
    }


<a id="org128b3a3"></a>

### Using with \`init\`

This way has Less platforms support.

    use ngrs::Runtime;
    
    fn main() {
        Runtime::initialize();
        let runtime = Runtime::new();
    
        // Your Guile-dependent code here
    }


<a id="org5b624fe"></a>

### Using with \`with<sub>guile</sub>\`

For more control over the Guile context:

    use ngrs::with_guile;
    
    fn main() {
        Runtime::initialize();
    
        with_guile(|vm:Runtime| {
            // Your Guile-dependent code here
        });
    }

