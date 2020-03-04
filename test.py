elems = [["John","France"], ["Mike", "France"], ["Ana","Italy"], ["Margarita","Italy"]]
found = set()
delete = [ ]
for i, e in enumerate(elems):
    if e[1] in found:
        delete.append(i)
    else:
        found.add(e[1])
for i, d in enumerate(delete):
    del elems[d-i]
print(elems)
