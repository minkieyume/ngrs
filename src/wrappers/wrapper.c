// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
#include <libguile.h>
#include "wrapper.h"

// 把所有宏转成函数
int ngrs_is_false(SCM x) { return scm_is_false(x); }
int ngrs_is_true(SCM x) { return scm_is_true(x); }
SCM ngrs_from_bool(int x) { return scm_from_bool(x); }
int ngrs_is_null(SCM x) { return scm_is_null(x); }
int ngrs_is_eq(SCM x, SCM y) { return scm_is_eq(x, y); }
int ngrs_is_symbol(SCM x) { return scm_is_symbol(x); }
SCM ngrs_eol() {return SCM_EOL;}
SCM ngrs_unspecified() { return SCM_UNSPECIFIED; }
SCM ngrs_undefined() { return SCM_UNDEFINED; }
int ngrs_unbound(SCM x) { return SCM_UNBNDP(x); }
