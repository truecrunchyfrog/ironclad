# Tag Examples

## Select only the tagged line

```text
ordinary line
chosen line ~ic=pick
ordinary line
```

## Select three lines above

```text
line 1
line 2
line 3
current ~ic=history=(3L<-1L)
line 5
```

## Select until a text boundary

```text
before
START
this is the part you want
~ic=chunk=('START'|<-1L)
after
```

## Select to the right until a marker

```text
~ic=menu=(1L->|'dessert')
soup
salad
dessert
cake
```

The tag operation removes the tag text itself from the output.
