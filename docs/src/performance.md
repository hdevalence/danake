# Performance

Performance numbers are TBD based on implementation work. However, early
prototypes suggest the following approximate server-side costs:

* Wallet topup: one set membership query plus less than 9 million Skylake cycles (3ms at 3GHz).
* Token spend: one set membership query plus less than 6 million Skylake cycles (2ms at 3GHz).