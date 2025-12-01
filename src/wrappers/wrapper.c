// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
#include <libguile.h>
#include "wrapper.h"

// 把所有宏转成函数
int ngrs_is_false(SCM x) { return scm_is_false(x); }
int ngrs_is_true(SCM x) { return scm_is_true(x); }
SCM ngrs_from_bool(int x) { return scm_from_bool(x); }
