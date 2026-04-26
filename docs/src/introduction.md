# Introduction

{{#include ../../README.md:3:3}}

Modern application software typically makes a lot of assumptions to work correctly.
Some of these assumptions can be ensured using unit, integration, and end-to-end testing.
However, more silent changes, such as discrete updates to documentation, is rarely covered by those tests.

Ironclad tracks what you rely on, and lets you know when it breaks your expectations.

## How it works

The regular routine is:
- The current state is evaluated.
- Compare the current state to the baseline.
- If different, the changes are reviewed by you.
- If you approve the changes, they are promoted to the baseline.
