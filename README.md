# Repo Demonstrating `grouped_by` bug

1. Clone the repo
2. Ensure you've got Rust installed
3. Run `bin/run`
4. Change the order of foreign keys on `Transaction` (parts of it should work
   when `merchant_id` is third in the struct)

This should result in each merchant listed having one corresponding transaction.

Reordering the attributes on the `Transaction` struct within `src/models.rs`
impacts the counts of grouped transactions. In certain cases, it seems like it
chooses the second of the `*_id` foreign keys, but there's less of a pattern
there. Changing the order of e.g. `belongs_to(Category)` doesn't have an
impact, only the order of the `*_id`s.

It seems like there's an issue in the `foreign_key()` call within `grouped_by`,
but it's not obvious what the problem is.
