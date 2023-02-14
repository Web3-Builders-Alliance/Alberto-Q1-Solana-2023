What are we going to use the deposit program for?
Instructions for deposit program
WBA video Tuesday 2/7 minute 1:08

Once you have your code ready
anchor build
anchor deploy will give the program ID needed to replace generic

##2/14-Class 7
is the SPL build on native? and Anchor helps by wrapping around those programs

Token 2022 is an updated version

you are using `use anchor_spl` as a way to call the wrapper, rather than wrapper the program line by line?
-   so the program is the same, it adds the the anchor wrapper because it was called in the anchor_spl
why do mean normal token account?

why would you wrapped a token? sol on one side, and have usdc on the other side, isnt spl native?

in the ts file who can you tell the difference between spl and native?
How are you connecting between rs and ts file?

is that a good idea to keep it with the unknow?
well there is execute to have it name it.
the IDL is powerful to see other people program

The video start byb talking about the Associated Token Account
1:42-TokenAccount is one of the few places where init_if_needed is helpful
3:50-Talking about hubbleprotocol Token2022
7:43-SPL Token TS
11:01-Talking about block explorer, solana explorer
13:22-Deploting IDL, it is worth including this in your test if you decide to use a local Explorer
14:45-QA
15:23-spl code example
15:41- shows cargo toml file, there are some dependencies that don't match with my code
20:01 to 23:21 -talking about sync_native special spl token account that wraps native, you will wrap a token for a AMM. Ideally you will build it for spl and you will switch back to native. Another example would ne derirative tokens, DT can be issue.
33:30-Solana explorer, and information about the ixs and why program says "unknow"
38:45- executes the `execSync` IDL from ts file, should be located right after sol_val, doing do helps logs a more detailed view in the explorer, explainig what each ix does and
45:35 - coding challenge for the week
51:00 to 1:23:00 - breakrooms (no recording)


Coding Challenge 2/14
-