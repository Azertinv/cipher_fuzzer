{
  "ciphers": [
    {
      "type": "Repeater",
      "key": [
        25,
        10
      ],
      "cipher_factory": "ShiftFactory"
    },
    {
      "type": "Progressor",
      "size": 3,
      "increment": -2,
      "offset": 2,
      "cipher_factory": "ShiftFactory"
    },
    {
      "type": "Progressor",
      "size": 3,
      "increment": -1,
      "offset": -3,
      "cipher_factory": "ShiftFactory"
    }
  ]
}
