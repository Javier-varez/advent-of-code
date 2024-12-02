#!/usr/bin/env python3

import hashlib as hl

prefix = 'abcdef'
string = prefix + str(609043)
md5 = hl.md5(string.encode('utf-8'))
print(string.encode('utf-8'))
print(md5.digest().hex())

prefix = 'ckczppom'

for i in range(0, 100000000):
    string = prefix + str(i)
    md5 = hl.md5(string.encode('utf-8'))
    if md5.digest().hex()[:6] == '000000':
        print(string.encode('utf-8'))
        print(md5.digest().hex())
        break
