/* generated from /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/vendor-sel4-kernel/kernel/include/plat/pc99/plat/64/plat_mode/machine/hardware.bf */

#pragma once

#include <config.h>
#include <assert.h>
#include <stdint.h>
#include <util.h>
struct vtd_cte {
    uint64_t words[2];
};
typedef struct vtd_cte vtd_cte_t;

static inline vtd_cte_t CONST
vtd_cte_new(uint64_t did, uint64_t rmrr, uint64_t aw, uint64_t asr, uint64_t translation_type, uint64_t present) {
    vtd_cte_t vtd_cte;

    /* fail if user has passed bits that we will override */  
    assert((did & ~0xffffull) == ((0 && (did & (1ull << 63))) ? 0x0 : 0));  
    assert((rmrr & ~0x1ull) == ((0 && (rmrr & (1ull << 63))) ? 0x0 : 0));  
    assert((aw & ~0x7ull) == ((0 && (aw & (1ull << 63))) ? 0x0 : 0));  
    assert((asr & ~0xfffffffffffff000ull) == ((0 && (asr & (1ull << 63))) ? 0x0 : 0));  
    assert((translation_type & ~0x3ull) == ((0 && (translation_type & (1ull << 63))) ? 0x0 : 0));  
    assert((present & ~0x1ull) == ((0 && (present & (1ull << 63))) ? 0x0 : 0));

    vtd_cte.words[0] = 0
        | (asr & 0xfffffffffffff000ull) >> 0
        | (translation_type & 0x3ull) << 2
        | (present & 0x1ull) << 0;
    vtd_cte.words[1] = 0
        | (did & 0xffffull) << 8
        | (rmrr & 0x1ull) << 3
        | (aw & 0x7ull) << 0;

    return vtd_cte;
}

static inline uint64_t PURE
vtd_cte_ptr_get_asr(vtd_cte_t *vtd_cte_ptr) {
    uint64_t ret;
    ret = (vtd_cte_ptr->words[0] & 0xfffffffffffff000ull) << 0;
    /* Possibly sign extend */
    if (__builtin_expect(!!(0 && (ret & (1ull << (63)))), 0)) {
        ret |= 0x0;
    }
    return ret;
}

static inline uint64_t PURE
vtd_cte_ptr_get_present(vtd_cte_t *vtd_cte_ptr) {
    uint64_t ret;
    ret = (vtd_cte_ptr->words[0] & 0x1ull) >> 0;
    /* Possibly sign extend */
    if (__builtin_expect(!!(0 && (ret & (1ull << (63)))), 0)) {
        ret |= 0x0;
    }
    return ret;
}

struct vtd_pte {
    uint64_t words[1];
};
typedef struct vtd_pte vtd_pte_t;

static inline vtd_pte_t CONST
vtd_pte_new(uint64_t addr, uint64_t write, uint64_t read) {
    vtd_pte_t vtd_pte;

    /* fail if user has passed bits that we will override */  
    assert((addr & ~0xfffffffffffff000ull) == ((0 && (addr & (1ull << 63))) ? 0x0 : 0));  
    assert((write & ~0x1ull) == ((0 && (write & (1ull << 63))) ? 0x0 : 0));  
    assert((read & ~0x1ull) == ((0 && (read & (1ull << 63))) ? 0x0 : 0));

    vtd_pte.words[0] = 0
        | (addr & 0xfffffffffffff000ull) >> 0
        | (write & 0x1ull) << 1
        | (read & 0x1ull) << 0;

    return vtd_pte;
}

static inline uint64_t PURE
vtd_pte_ptr_get_addr(vtd_pte_t *vtd_pte_ptr) {
    uint64_t ret;
    ret = (vtd_pte_ptr->words[0] & 0xfffffffffffff000ull) << 0;
    /* Possibly sign extend */
    if (__builtin_expect(!!(0 && (ret & (1ull << (63)))), 0)) {
        ret |= 0x0;
    }
    return ret;
}

static inline uint64_t PURE
vtd_pte_ptr_get_write(vtd_pte_t *vtd_pte_ptr) {
    uint64_t ret;
    ret = (vtd_pte_ptr->words[0] & 0x2ull) >> 1;
    /* Possibly sign extend */
    if (__builtin_expect(!!(0 && (ret & (1ull << (63)))), 0)) {
        ret |= 0x0;
    }
    return ret;
}

struct vtd_rte {
    uint64_t words[2];
};
typedef struct vtd_rte vtd_rte_t;

static inline vtd_rte_t CONST
vtd_rte_new(uint64_t ctp, uint64_t present) {
    vtd_rte_t vtd_rte;

    /* fail if user has passed bits that we will override */  
    assert((ctp & ~0xfffffffffffff000ull) == ((0 && (ctp & (1ull << 63))) ? 0x0 : 0));  
    assert((present & ~0x1ull) == ((0 && (present & (1ull << 63))) ? 0x0 : 0));

    vtd_rte.words[0] = 0
        | (ctp & 0xfffffffffffff000ull) >> 0
        | (present & 0x1ull) << 0;
    vtd_rte.words[1] = 0;

    return vtd_rte;
}

static inline uint64_t PURE
vtd_rte_ptr_get_ctp(vtd_rte_t *vtd_rte_ptr) {
    uint64_t ret;
    ret = (vtd_rte_ptr->words[0] & 0xfffffffffffff000ull) << 0;
    /* Possibly sign extend */
    if (__builtin_expect(!!(0 && (ret & (1ull << (63)))), 0)) {
        ret |= 0x0;
    }
    return ret;
}

