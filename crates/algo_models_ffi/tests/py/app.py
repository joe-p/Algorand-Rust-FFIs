from algo_models_ffi import decode_payment, encode_payment
from pprint import pprint


def main():
    tx_u8 = [
        137,
        163,
        97,
        109,
        116,
        206,
        0,
        15,
        66,
        64,
        163,
        102,
        101,
        101,
        205,
        3,
        232,
        162,
        102,
        118,
        205,
        3,
        232,
        163,
        103,
        101,
        110,
        172,
        116,
        101,
        115,
        116,
        110,
        101,
        116,
        45,
        118,
        49,
        46,
        48,
        162,
        103,
        104,
        196,
        32,
        72,
        99,
        181,
        24,
        164,
        179,
        200,
        78,
        200,
        16,
        242,
        45,
        79,
        16,
        129,
        203,
        15,
        113,
        240,
        89,
        167,
        172,
        32,
        222,
        198,
        47,
        127,
        112,
        229,
        9,
        58,
        34,
        162,
        108,
        118,
        205,
        5,
        220,
        163,
        114,
        99,
        118,
        196,
        32,
        247,
        150,
        167,
        185,
        149,
        130,
        87,
        172,
        85,
        147,
        5,
        104,
        184,
        87,
        44,
        206,
        68,
        62,
        179,
        88,
        173,
        129,
        245,
        68,
        228,
        113,
        240,
        228,
        211,
        170,
        89,
        244,
        163,
        115,
        110,
        100,
        196,
        32,
        154,
        33,
        203,
        60,
        169,
        134,
        36,
        253,
        73,
        75,
        144,
        135,
        105,
        171,
        72,
        68,
        33,
        226,
        186,
        94,
        9,
        240,
        140,
        12,
        209,
        166,
        101,
        40,
        185,
        34,
        235,
        246,
        164,
        116,
        121,
        112,
        101,
        163,
        112,
        97,
        121,
    ]
    # convert tx_u8 to bytes
    tx_bytes = bytes(tx_u8)
    print(tx_bytes)
    decoded_tx = decode_payment(tx_bytes)
    pprint(decoded_tx.__dict__)
    encoded_tx = encode_payment(decoded_tx)
    assert encoded_tx[2:] == tx_bytes
    print("success")


if __name__ == "__main__":
    main()
