# <p align=center> **psm** - <i>CLI password manager written in Rust</i> </p>

## What is this tool?

psm can save your passwords and encrypt them with aes encryption standard. this tool tend to be easy and minimal as possible

Notice: this tool is not complete, you can create passwords and get them again but there is more implementations for this tool
Then please use the stable (master) branch.

## How it works?

_(Dumped from the command 'psm info')_

First you need to initilize an origin, you can it by typing:

`psm init`

Then you need to copy your key and save it somemwhere.

_**(Notice if you forgot that key, you will not be able to see your passwords again!)**_

After that you can add your first password, by typing:

`psm create name <your-password-name> password <your-actuall-password> description <your-password description> key <your-ley>`

Then you can decrypr/see your password by typing:

`psm get name <your-password-name> key <your-key>`

In the end if you already have an origin and you want to start over, you can do:
`psm remove all`

(Notice this will delete all your current passwords)

---

If you have any idea, I would be happy to share it with me.
