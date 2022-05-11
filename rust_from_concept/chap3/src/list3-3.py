x = 1
y = x

print("x=",x,id(x))
print("y=",y,id(y))

x = x + 1 # xのみの値が変わる。yは変わらない。

print("x=",x,id(x))
print("y=",y,id(y))

y += 1 # yから遡ってxにも反映される。

print("x=",x,id(x))
print("y=",y,id(y))

x = [0,1,2]
y = x

print("x=",x,id(x))
print("y=",y,id(y))

x.append(3) # xとyはリストに対するラベルで、同じオブジェクトを参照しているので、値が変わる。

print("x=",x,id(x))
print("y=",y,id(y))
