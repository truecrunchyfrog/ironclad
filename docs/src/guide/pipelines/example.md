# Example

Say that you have a directory containing your diary:

```
my-diary
├── 1970-01-01.json
├── 1970-01-02.json
├── 1970-01-03.json
├── ...
├── 2026-04-24.json
├── 2026-04-25.json
└── 2026-04-26.json
```

Each file is a JSON object with:
- `thoughts`: your thoughts about how the day went.
- `rating`: a measurement of how bad the day was.

```json
// my-diary/2026-04-26.json
{
  "thoughts": "Nothing remarkable today.",
  "rating": 0.5
}
```

Now, what if you want to ensure that nobody tampers with your ratings?

To begin with, you can make a pipeline that:
- reads the files in `my-diary`,
- selects the value of each file's `rating`.

Running that pipeline would yield the output:
```json
{
  "2026-04-06.json": 0.5,
  "2026-04-05.json": 0.48,
  "2026-04-04.json": 0.52,
  ...
  "1970-01-03.json": 0.49,
  "1970-01-02.json": 0.43,
  "1970-01-01.json": 0.51
}
```

Given the input (in this case, the files in `my-diary`) remains the same, the output will too.

- Adding or removing a file with a `rating` will affect the output, by adding or removing an entry.
- Changing a `rating` will affect the output, by changing an entry's value.

However, since the pipeline only selects the `rating`, other changed input does not affect the output.
- Adding or removing a file without a `rating` will not affect the output.
- Changing files with or without a `rating` without touching the rating, such as by only changing a file's `thoughts`, will not affect the output.

In Ironclad, the entire output of a pipeline is called a batch, and each output item is called a sample.
The representation is simplified.

```json
{ // a batch
  "2026-04-06.json": 0.5,  // a sample
  "2026-04-05.json": 0.48, // a sample
  "2026-04-04.json": 0.52, // a sample
  ...
  "1970-01-03.json": 0.49, // a sample
  "1970-01-02.json": 0.43, // a sample
  "1970-01-01.json": 0.51  // a sample
}
```
