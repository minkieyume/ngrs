// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
#ifndef GUILE_WRAPPER_H
#define GUILE_WRAPPER_H

#include <libguile.h>  // 自动从 include_paths 中找

// 你的包装函数声明
int ngrs_is_false(SCM x);
int ngrs_is_true(SCM x);
SCM ngrs_from_bool(int x);
int ngrs_is_null(SCM x);
int ngrs_is_eq(SCM x, SCM y);
int ngrs_is_symbol(SCM x);

#endif
