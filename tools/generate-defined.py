for i in range(1, 33):
    bit_width = i * 8
    print("define_fixed_bytes!(Bytes%s, U%s, S%s);" % (i, bit_width, bit_width))

print()

for i in range(1, 33):
    bit_width = i * 8
    print("define_uint!(U%s, Bytes%s, S%s);" % (bit_width, i, bit_width))

print()

for i in range(1, 33):
    bit_width = i * 8
    print("define_sint!(S%s, Bytes%s, U%s);" % (bit_width, i, bit_width))
