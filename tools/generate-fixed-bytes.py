for i in range(1, 33):
    bit_width = i * 8
    print("define_fixed_bytes!(Bytes%s, U%s, S%s);" % (i, bit_width, bit_width))
