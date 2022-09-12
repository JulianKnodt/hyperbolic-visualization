# Hyperbol

Hyperbol is an exploration of tree visualizations!

Trees are often visualized flatly. That is, nodes of children are 2x the distant from their
parent as they are from their grandparent. This makes it difficult to view very large graphs, as
the _visual_ size of the graph grows linearly with the depth of the tree. As a viewer, it
becomes extremely hard to see large and dense graphs. Instead, we can have the distance of
childrens grow logarithmically.  That is, the distance of a node to its parent is 1/k of the
distance from the parent to the grandparent, with k usually equal to 2.

This kind of visualization is called [hyperbolic
trees](https://en.wikipedia.org/wiki/Hyperbolic_tree), where hyperbolic refers to an embedding
in hyperbolic space. This has the effect of having a strong fisheye effect around the origin, so
things at infinity are visible on the corner of the visualization.

# TODOs

- Create a file explorer with a TUI
