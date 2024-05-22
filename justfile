alias d := documentation
alias doc := documentation

# build documentation
documentation:
    typst compile docs/documentation.typ documentation.pdf
