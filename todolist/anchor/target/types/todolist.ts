/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/todolist.json`.
 */
export type Todolist = {
  "address": "6u7Wzgps8X8Qjd5AaqaF5mpKdfZzSfNt2MaPjATf2Z6Y",
  "metadata": {
    "name": "todolist",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addTask",
      "discriminator": [
        234,
        40,
        30,
        119,
        150,
        53,
        76,
        83
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "todolist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "listName"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "task",
          "type": "string"
        }
      ]
    },
    {
      "name": "completeTask",
      "discriminator": [
        109,
        167,
        192,
        41,
        129,
        108,
        220,
        196
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "todolist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "listName"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "taskIndex",
          "type": "u32"
        }
      ]
    },
    {
      "name": "initializeList",
      "discriminator": [
        79,
        19,
        174,
        114,
        45,
        41,
        84,
        106
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "todolist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "listName"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "listName",
          "type": "string"
        }
      ]
    },
    {
      "name": "removeTask",
      "discriminator": [
        129,
        98,
        0,
        238,
        73,
        182,
        74,
        3
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "todolist",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "listName"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "taskIndex",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "list",
      "discriminator": [
        169,
        24,
        186,
        110,
        22,
        139,
        190,
        82
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "taskNotFound",
      "msg": "Task not found in the ToDoList."
    }
  ],
  "types": [
    {
      "name": "list",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "listName",
            "type": "string"
          },
          {
            "name": "tasks",
            "type": {
              "vec": {
                "defined": {
                  "name": "task"
                }
              }
            }
          },
          {
            "name": "taskCount",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "task",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "isCompleted",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
