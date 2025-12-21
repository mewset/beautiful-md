\#No Space After Hash
\##Another One


### Too Many Spaces

\####Trailing Hash####
\#####Mixed   Spacing#####

Some text here.

Too many blank lines above.

| \   | Name  | Age | City      |
| \   | Alice | 30  | Stockholm |
| \   | Bob   | 25  |

Table with missing cells above ^

| \   | Header1 | Header2 | Header3 |
| \   | ------- | ------- |
| \   | Only    | Two     | Cells   | Per | Row |

Uneven columns ^

| Name            | Age |
| --------------- | --- |
| No opening pipe |

| Name                 | Age | City      |
| -------------------- | --- | --------- |
| Missing closing pipe | 25  | Stockholm |
| Bob                  | 30  | Göteborg  |

| \   | Weird   | Alignment |
| \   | :-----: | --------: |
| \   | Center? | Right     |

- Item 1

- Item 2 different marker

- Item 3 another marker
  - Nested with 2 spaces
    - Nested with 4 spaces
      - Nested with weird indent

- Back to root
  1. Ordered nested
  2. All ones
  3. Mix of numbers
    - Unordered inside ordered
      1. Deep nesting
    - Wrong indent back

1. Ordered
2. Wrong number
3. Reset
4. Big number

````rust
Code without closing fence

```
Different fence style
````

Mismatched closing fence

```
Quad backticks
````

```
No language tag
Some code
````

```
Fence with trailing space
`

Table inside list:

| * Item | Col1 | Col2 |
| ------ | ---- |
| A      | B    |

Inline `code with |pipe|` in it

| Table | With `code`  | And **bold** |
| ----- | ------------ | ------------ |
| Mixed | *formatting* | ~~strike~~   |


# Heading with [link](http://example.com)


## Heading with `code`


### Heading with **bold** and *italic*

 > 
 > Quote
 > 
 >  > 
 >  > Nested quote
 >  > Back
 >  > 
 >  >  > 
 >  >  > Deep nest
 >  >  > Wrong level back

 > 
 > Quote with
 > no continuation marker
 > should it continue?

- [ ] Task list unchecked
- [x] Task list checked
- [x] Uppercase X
- [ ] Normal
  -\[ \] No space
- \[\]Missing x

- [x] Different marker

\#No space before heading after paragraph
Text right before heading

Text with trailing spaces
And more trailing spaces

Mixed	tabs	and    spaces

| Header with emoji 🎉    | Unicode ñ  | Math ∑   |
| ---------------------- | ---------- | -------- |
| Test                   | Test       | Test     |

| Spaces | Around | Pipes |
| ------ | ------ | ----- |
| A      | B      | C     |

| \   | No   | Alignment | Row |
| \   | Data | Here      | Now |

Very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very long line without breaks

| Cell with \ | escaped pipe | Normal |
| ----------- | ------------ |
| Data        | More         |

<!-- HTML comment -->

<div>HTML block</div>
<span>Inline HTML</span>

\*\*Bold not closed
\*Italic not closed
~~Strike not closed

**Bold with
linebreak**

\[Link with no URL\]
[Link](no-protocol.com)
![Image with no alt\]
![](no-src)


## Horizontal rules with different styles:

---

---

---

---


#


##

Just hashes

Text#notaheading
\#notaheading either

- List
  Continuation without marker

Text

- List starting abruptly

1. Very high starting number
2. Continue
3. Go back?

Table with alignment and weird spacing:

| Left | Center | Right |
| :--- | :----: | ----: |
| A    | B      | C     |
| D    | E      | F     |

Nested structures:

 > 
 > Quote with list:
 > 
 > * Item 1
 > * Item 2
 >   * Nested
 > 
 > And table:
 > 
| >   | A   | B   |
| >   | --- | --- |
| >   | 1   | 2   |

Empty table cells:

|
| ----- | --- |
| value |
| value |
|

Multiple heading levels mixed:


# H1


### H3 (skipped H2)


## H2 (going back)


##### H5 (big jump)


# H1 again

Weird list indentation:
-Item no space

- 1 space
- 2 spaces
- 3 spaces
  - 4 spaces
  - 5 spaces
- Back to normal

Code blocks nested in quotes:

 > 
 > Some quote
 > 
 > ````
 > code in quote
 > ````
 > 
 > End quote

Backslash escapes everywhere:
\# Not a heading
- Not a list
\| Not a table
\> Not a quote
\` Not code

Links and formatting:
**[Bold link](http://example.com)**
*[Italic link](http://example.com)*
[Link with **bold** inside](http://example.com)

Table with very uneven content:

| Short | Very very very very very very very very very very long content | S   |
| ----- | -------------------------------------------------------------- | --- |
| A     | B                                                              | C   |

Mixed line endings and no final newline
