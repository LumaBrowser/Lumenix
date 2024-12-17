use std::collections::HashMap;

// html parser
#[derive(Debug)]
struct Node {
   node_type: NodeType,
   children: Vec<Node>,
   attributes: HashMap<String, String>,
}

#[derive(Debug)]
enum NodeType {
   Element(String),
   Text(String),
}

struct Parser {
   pos: usize,
   input: String,
}

impl Parser {
   fn new(input: String) -> Parser {
      Parser {
         pos: 0,
         input,
      }
   }

   fn consume_whitespace(&mut self) {
      while self.pos < self.input.len() && self.input[self.pos..].starts_with(char::is_whitespace) {
         self.pos += 1;
      }
   }

   fn parse_tag_name(&mut self) -> String {
      let mut name = String::new();
      while self.pos < self.input.len() {
         let char_at_pos = self.input[self.pos..].chars().next().unwrap();
         if char_at_pos.is_alphanumeric() {
            name.push(char_at_pos);
            self.pos += char_at_pos.len_utf8();
         } else {
            break;
         }
      }
      name
   }

   fn parse_node(&mut self) -> Node {
      match self.input[self.pos..].chars().next().unwrap() {
         '<' => self.parse_element(),
         _ => self.parse_text(),
      }
   }

   fn parse_text(&mut self) -> Node {
      let mut text = String::new();
      while self.pos < self.input.len() && !self.input[self.pos..].starts_with('<') {
         text.push(self.input[self.pos..].chars().next().unwrap());
         self.pos += 1;
      }
      Node {
         node_type: NodeType::Text(text),
         children: vec![],
         attributes: HashMap::new(),
      }
   }

   fn parse_element(&mut self) -> Node {
      // consume the opening < nomnom
      assert_eq!(self.input[self.pos..].chars().next().unwrap(), '<');
      self.pos += 1;

      let tag_name = self.parse_tag_name();
      let mut attributes = self.parse_attributes();

      // consume the opening > because lumenix is hungry
      assert_eq!(self.input[self.pos..].chars().next().unwrap(), '>');
      self.pos += 1;

      // parse children
      let mut children = vec![];
      loop {
         self.consume_whitespace();
         if self.pos >= self.input.len() {
            break;
         }
         if self.input[self.pos..].starts_with("</") {
            // consume closing tag
            // damn its REALLY hungry
            self.pos += 2;
            let close_tag_name = self.parse_tag_name();
            assert_eq!(tag_name, close_tag_name);
            assert_eq!(self.input[self.pos..].chars().next().unwrap(), '>');
            self.pos += 1;
            break;
         }
         children.push(self.parse_node());
      }

      Node {
         node_type: NodeType::Element(tag_name),
         children,
         attributes,
      }
   }

   fn parse_attributes(&mut self) -> HashMap<String, String> {
      let mut attributes = HashMap::new();

      loop {
         self.consume_whitespace();
         if self.pos >= self.input.len() || self.input[self.pos..].starts_with('>') {
            break;
         }

         // parse attribute name
         let name = self.parse_tag_name();
         if name.is_empty() {
            break;
         }

         // parse '='
         assert_eq!(self.input[self.pos..].chars().next().unwrap(), '=');
         self.pos += 1;

         // parse attribute value
         assert_eq!(self.input[self.pos..].chars().next().unwrap(), '"');
         self.pos += 1;
         let mut value = String::new();
         while !self.input[self.pos..].starts_with('"') {
            value.push(self.input[self.pos..].chars().next().unwrap());
            self.pos += 1;
         }
         self.pos += 1;

         attributes.insert(name, value);
      }

      attributes
   }
}

// layout engine
#[derive(Debug)]
struct Dimensions {
   content: Rect,
   padding: EdgeSizes,
   border: EdgeSizes,
   margin: EdgeSizes,
}

#[derive(Debug)]
struct Rect {
   x: f32,
   y: f32,
   width: f32,
   height: f32,
}

#[derive(Debug)]
struct EdgeSizes {
   left: f32,
   right: f32,
   top: f32,
   bottom: f32,
}

#[derive(Debug)]
struct LayoutBox {
   dimensions: Dimensions,
   box_type: BoxType,
   children: Vec<LayoutBox>,
}

#[derive(Debug)]
enum BoxType {
   BlockNode(Node),
   InlineNode(Node),
   AnonymousBlock,
}

impl LayoutBox {
   fn new(box_type: BoxType) -> LayoutBox {
      LayoutBox {
         dimensions: Dimensions {
            content: Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            padding: EdgeSizes { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
            border: EdgeSizes { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
            margin: EdgeSizes { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
         },
         box_type,
         children: vec![],
      }
   }

   fn layout(&mut self, containing_block: Dimensions) {
      match self.box_type {
         BoxType::BlockNode(_) => self.layout_block(containing_block),
         BoxType::InlineNode(_) => self.layout_inline(containing_block),
         BoxType::AnonymousBlock => self.layout_anonymous(containing_block),
      }
   }

   fn layout_block(&mut self, containing_block: Dimensions) {
      // calculate width
      self.calculate_block_width(containing_block);

      // calculate position
      self.calculate_block_position(containing_block);

      // layout children
      self.layout_block_children();

      // calculate height
      self.calculate_block_height();
   }

   fn calculate_block_width(&mut self, containing_block: Dimensions) {
      let style = match self.box_type {
         BoxType::BlockNode(ref node) => node,
         _ => return,
      };

      // for simplicity, we'll use a default width for now
      // TO:DO - change this
      self.dimensions.content.width = containing_block.content.width * 0.8;
   }

   fn calculate_block_position(&mut self, containing_block: Dimensions) {
      let style = match self.box_type {
         BoxType::BlockNode(ref node) => node,
         _ => return,
      };

      // simple vertical stacking
      self.dimensions.content.x = containing_block.content.x + self.dimensions.margin_left;
      self.dimensions.content.y = containing_block.content.height;
   }

   fn layout_block_children(&mut self) {
      let d = self.dimensions.clone();
      for child in &mut self.children {
         child.layout(d.clone());

         // update parent height to account for child's height
         self.dimensions.content.height += child.dimensions.margin_box().height;
      }
   }

   fn calculate_block_height(&mut self) {
      // to be simple for now, height is just the sum of children's heights
      // TO:DO - change this
      if self.dimensions.content.height == 0.0 {
         self.dimensions.content.height = 50.0;
      }
   }

   fn layout_inline(&mut self, containing_block: Dimensions) {
      // simplified inline layout
      self.dimensions.content.width = containing_block.content.width;
      self.dimensions.content.height = 20.0;
   }

   fn layout_anonymous(&mut self, containing_block: Dimensions) {
      self.dimensions.content.width = containing_block.content.width;
      self.layout_block_children();
   }
}

// main browser engine structure
struct BrowserEngine {
   fn new(html: String) -> BrowserEngine {
      BrowserEngine {
         parser: Parser::new(html),
         dom: None,
         layout_root: None,
      }
   }

   fn parse(&mut self) {
      self.dom = Some(self.parser.parse_node());
   }

   fn create_layout_tree(&mut self) {
      if let Some(ref dom_root) = self.dom {
         self.layout_root = Some(self.build_layout_tree(dom_root));
      }
   }

   fn build_layout_tree(&self, node: &Node) -> LayoutBox {
      let mut root = LayoutBox::new(match node.node_type {
         NodeType::Element(_) => BoxType::BlockNode(node.clone()),
         NodeType::Text(_) => BoxType::InlineNode(node.clone()),
      });

      for child in &node.children {
         root.children.push(self.build_layout_tree(child));
      }

      root
   }

   fn layout(&mut self) {
      if let Some(ref mut layout_root) = self.layout_root {
         let containing_block = Dimensions {
            content: Rect {
               x: 0.0,
               y: 0.0,
               width: 800.0, 
               height: 600.0,
            },
            padding: EdgeSizes { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
            border: EdgeSizes { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
            margin: EdgeSizes { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
         };
         layout_root.layout(containing_block);
      }
   }
}

// load
fn main() {
   let html = String::from(
      "<div class=\"container\">
         <h1>Hello, Browser!</h1>
         <p>This is a simple page rendering in Lumenix.</p>
      </div>"
   );

   let mut browser = BrowserEngine::new(html);

   // parse html into dom
   browser.parse();
   println!("DOM created: {:?}", browser.dom);

   // create layout tree
   browser.create_layout_tree();

   // perform layout
   browser.layout();
   println!("Layout complete: {:?}", browser.layout_root);
}