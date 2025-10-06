// disable_asserts.h - Desactiva ASSERT macros de Vectrexy para testing
#pragma once

// Redefinir ASSERT para que no haga nada
#define ASSERT(condition) ((void)0)
#define ASSERT_MSG(condition, msg, ...) ((void)0)
#define FAIL() ((void)0)
#define FAIL_MSG(msg, ...) ((void)0)
