### Racket dependency visualization
This small programm is used to visualize the dependencies within a Racket project.
That is, the "require" statements from the source files are parsed and a graph is
generated. The graph is then outputted in markdown format. The mermaid library can 
then be used to render the graph in a markdown file.

### Usage
The program can be run by itself. It takes a single argument, the path to the project.
Unless the path is invalid, inaccessable or you did some hardlink schenanigans, it should
work. The output is written to stdout in markdown format.