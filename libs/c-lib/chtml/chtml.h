#ifndef CHTML_LIB

#define CHTML_LIB

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * Enumeration of HTML-Tags
 **/
typedef enum CHtmlTag {
  Div,
  P,
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Span,
  Section,
  Article,
  Audio,
  B,
  Br,
  Code,
  Em,
  Fieldset,
  Form,
  Img,
  Input,
  Label,
  Link,
  Li,
  Menu,
  Ol,
  Option,
  Style,
  Svg,
  Table,
  Td,
  Th,
  Ul,
  Video,
  Frag,
  Button,
  Script,
  Select,
  Aside,
  Nav,
  A,
  None,
} CHtmlTag;

/**
 * Standard Html Element Representation.
 *
 * Represents any Html-tag, along with their subsequent children.
 */
typedef struct CHtmlElement {
  /**
   * Kind of Html element
   */
  enum CHtmlTag tag;
  /**
   * Array of children
   */
  struct CHtml *children;
  /**
   * Amount of children
   */
  size_t len;
} CHtmlElement;

/**
 * Union of a Html Element, and Raw text.
 */
typedef union CHtmlContent {
  CHtmlElement element;
  const char *text;
} CHtmlContent;

typedef struct CHtml {
  CHtmlContent content;
  bool isElement;
} CHtml;

/**
 * Creates empty CHtmlElement Div
 */
CHtmlElement *e_div();

CHtmlContent *unionize(CHtmlElement *element, char *text);

/**
 * <div>
 *  <p>Hello, World!</p>
 * </div>
 */
CHtml simple_test();

/**
 * Recursively frees the given html node
 * TODO: Find out if this works.
 */
void free_chtml(CHtml *chtml);

#endif // !CHTML_LIB
