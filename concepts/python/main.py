# this is not documented in our documentation, but all you need is:
# - python 3.10.9 or later
# - requests installed (pip install requests)
# then you can run this file with python concepts/python/main.py

# please note this was originally just for me (katniny) to test concepts, 
# and is not the engine itself.
# this can only load raw html from websites and you may notice freezeups
# when loading large sites
import requests
from html.parser import HTMLParser
import tkinter as tk
from tkinter import ttk, Text
import re
from urllib.parse import urljoin, urlparse

class LumenHTMLParser(HTMLParser):
   def __init__(self):
      super().__init__()
      self.formatted_content = []
      self.links = []
      self.in_title = False
      self.title = ""
      self.current_tags = set()
      self.list_index = 0
        
   def handle_starttag(self, tag, attrs):
      self.current_tags.add(tag)
        
      if tag == "a":
         for attr in attrs:
            if attr[0] == "href":
               self.links.append(attr[1])
      elif tag == "title":
         self.in_title = True
      elif tag == "p":
         self.formatted_content.append(("p_start", None))
      elif tag == "h1":
         self.formatted_content.append(("h1_start", None))
      elif tag == "h2":
         self.formatted_content.append(("h2_start", None))
      elif tag == "li":
         self.list_index += 1
         self.formatted_content.append(("li_start", self.list_index))
      elif tag == "ul":
         self.formatted_content.append(("ul_start", None))
         self.list_index = 0
    
   def handle_endtag(self, tag):
      if tag in self.current_tags:
         self.current_tags.remove(tag)
            
      if tag == "title":
         self.in_title = False
      elif tag == "p":
         self.formatted_content.append(("p_end", None))
      elif tag == "h1":
         self.formatted_content.append(("h1_end", None))
      elif tag == "h2":
         self.formatted_content.append(("h2_end", None))
      elif tag == "li":
         self.formatted_content.append(("li_end", None))
      elif tag == "ul":
         self.formatted_content.append(("ul_end", None))
    
   def handle_data(self, data):
      if self.in_title:
         self.title += data
      if data.strip():
         self.formatted_content.append(("text", data.strip()))

class BrowserEngine:
   def __init__(self):
      self.current_url = None
      self.history = []
        
   def fetch_page(self, url):
      """fetch & parse page"""
      try:
         response = requests.get(url)
         response.raise_for_status()
         return response.text
      except requests.RequestException as e:
         return f"Error fetching page: {str(e)}"
    
   def parse_page(self, html_content):
      """parse html and extract useful info"""
      parser = LumenHTMLParser()
      parser.feed(html_content)
      return {
         "title": parser.title,
         "formatted_content": parser.formatted_content,
         "links": parser.links
      }

class BrowserGUI:
   def __init__(self):
      self.engine = BrowserEngine()
        
      # create window
      self.root = tk.Tk()
      self.root.title("Lumenix Browser Engine Test")
      self.root.geometry("800x600")
        
      # nav frame
      nav_frame = ttk.Frame(self.root)
      nav_frame.pack(fill=tk.X, padx=5, pady=5)
        
      # url entry
      self.url_entry = ttk.Entry(nav_frame)
      self.url_entry.pack(side=tk.LEFT, fill=tk.X, expand=True)
        
      # nav buttons
      ttk.Button(nav_frame, text="Go", command=self.navigate).pack(side=tk.LEFT)
      ttk.Button(nav_frame, text="Back", command=self.go_back).pack(side=tk.LEFT)
        
      # content area
      self.content_area = Text(self.root, wrap=tk.WORD, padx=10, pady=10)
      self.content_area.pack(fill=tk.BOTH, expand=True, padx=5, pady=5)
        
      # config text for formatting
      self.content_area.tag_configure("title", font=("Times New Roman", 18, "bold"))
      self.content_area.tag_configure("h1", font=("Times New Roman", 16, "bold"))
      self.content_area.tag_configure("h2", font=("Times New Roman", 14, "bold"))
      self.content_area.tag_configure("normal", font=("Times New Roman", 12))
      self.content_area.tag_configure("link", font=("Times New Roman", 12), foreground="blue", underline=1)

      self.content_area.configure(state='disabled')
    
   def render_content(self, parsed_content):
      """render formatted content"""
      self.content_area.configure(state='normal')
      self.content_area.delete(1.0, tk.END)
        
      if parsed_content["title"]:
         self.content_area.insert(tk.END, parsed_content["title"] + "\n\n", "title")
        
      current_tags = []
      for content_type, content in parsed_content["formatted_content"]:
         if content_type == "text":
            tags = tuple(current_tags) if current_tags else "normal"
            self.content_area.insert(tk.END, content + " ", tags)
         elif content_type == "p_start":
            self.content_area.insert(tk.END, "\n")
            current_tags = []
         elif content_type == "p_end":
            self.content_area.insert(tk.END, "\n")
         elif content_type == "h1_start":
            self.content_area.insert(tk.END, "\n")
            current_tags = ["h1"]
         elif content_type == "h2_start":
            self.content_area.insert(tk.END, "\n")
            current_tags = ["h2"]
         elif content_type == "li_start":
            self.content_area.insert(tk.END, f"\n  • ")
         elif content_type == "ul_start":
            self.content_area.insert(tk.END, "\n")
        
      self.content_area.configure(state='disabled')
    
   def navigate(self):
      """nav to url"""
      url = self.url_entry.get()
      if not (url.startswith('http://') or url.startswith('https://')):
         url = "https://" + url
        
      html_content = self.engine.fetch_page(url)
      if html_content:
         parsed_content = self.engine.parse_page(html_content)
         self.render_content(parsed_content)
            
         # store in history
         self.engine.history.append(url)
         self.engine.current_url = url
    
   def go_back(self):
      """nav to previous page"""
      if len(self.engine.history) > 1:
         self.engine.history.pop()
         previous_url = self.engine.history[-1]
         self.url_entry.delete(0, tk.END)
         self.url_entry.insert(0, previous_url)
         self.navigate()
    
   def run(self):
      """start browser"""
      self.root.mainloop()

if __name__ == "__main__":
   browser = BrowserGUI()
   browser.run()