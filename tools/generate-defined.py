for i in range(1, 33):
    st = "define_fixed_bytes!(Bytes%s" % (i,)    
    for j in range(1, i + 1):
        bit_width = j * 8
        st += ", U%s" % (bit_width, )
        st += ", S%s" % (bit_width, )

    st += ");"
    print(st)


print()

for i in range(1, 33):
    bit_width = i * 8
    st = "define_uint!(U%s" % (bit_width,)
    for j in range(1, i + 1):
        bit_width = j * 8
        st += ", Bytes%s" % (j, )
        st += ", S%s" % (bit_width, )

    st += ");"
    print(st)

print()

for i in range(1, 33):
    bit_width = i * 8
    st = "define_sint!(S%s" % (bit_width,)
    for j in range(1, i + 1):
        bit_width = j * 8
        st += ", Bytes%s" % (j, )
        st += ", U%s" % (bit_width, )

    st += ");"
    print(st)

print()

st = "define_address_from!(";
for i in range(1, 21):
    bit_width = i * 8
    st += "Bytes%s, U%s, S%s," % (i, bit_width, bit_width)
st += ");"

print(st)

