# Sources folder

This folder will be used primarily for the design purposes of the communication architecture between front-end and backend.

## Backend Service structure

This is how the microservices should be split, along with a few key components of their functionality
All operate around the same dataset, but each serves a specific purpose, so they don't overlap their responisibilites.

```txt
LEGEND:

FE: Front End
BE: Back End
x => y: from x to y
REQ: Request
EV: Event
```

```txt
PROXY
│
├─── BE for SITE_RENDERING
│    │
│    ├ get_all_games(live / previous / upcoming / all)
│    ├ get_matches(live / previous / upcoming / all)
│    ├ get_game_matches(game, live / previous / upcoming / all)
│    │
│    ├ get_ticket() # for currently logged in user, current open ticket 
|    ├ get_latest_tickets()
│    │
│    ├
│    ├ get_users_tickets()
│    ├ get_users_profile()
│    ├ get_users_statistics()
│    └ get_users_invoices()
│

```
<!--

OLD:
Backend Proxy
│
│    USER_SERVICE
│    ├ change username    *REQ FE => BE
│    ├ change email    *REQ FE => BE
│    └ disable / delete account    *REQ FE => BE
│
│    ADMIN_SERVICE
├─── administration service {accessing backend}
│    ├ creating matches    *REQ FE => BE
│    ├ correcting / tweaking bet ratios    *REQ FE => BE
│    ├ editing match parameters    *REQ FE => BE
│    └ removing matches    *REQ FE => BE
│
│    MATCH_SERVICE
├─── match handling service
|    ├ match starting loop
|    ├ notify clients / update client state - match started {frontend}    *EV BE => FE
│    ├ show all matches {for fronend}    *REQ FE => BE
│    └ show match info {for frontend}    *REQ FE => BE
│
│    BET_SERVICE
├─── betting service
│    ├ open a match (if the match is available) {for frontend}    *REQ FE => BE
│    ├ make a bet     *REQ FE => BE
│    │  └ calculate new bet ratio, update internal state of the match
│    ├ update current bet ratio {for frontend}    *EV BE => FE
│    └ update live bet feed {for frontend}    *EV BE => FE
│
│    INV_SERVICE
└─── invoice service
     ├ show list of invoices {for frontend}    *REQ FE => BE
     ├ show invoice {for frontend}    *REQ FE => BE
     └ pay an invoice     *REQ FE => BE
-->