# DB layer

## Game

Frontend zobrazuje hry v leve liste a pri vypsanych zapasech.

Potrebuju umet:
- vytvorit hru
- listnout vsechny hry

## Game Match

Frontend zobrazuje zapsay ve 3 tabech podle `GameMatchEventType`.

Potrebuju umet:
- vytvorit zapas
- ziskat zapasy podle `GameMatchEventType` a id hry
- vytvorit `GameMatchEventType`

## Team

Neni potreba umet prirazovat teamy ke hram,
protoze na frontendu nemusime hledat teamy podle her,
ale zapasy, ktere se s game id uz inicializuji.

Potrebuju umet:
- vytvorit team
- ziskat team podle id

## Bet

Sazky se nemusi vazat

Potrebuju umet:
- vytvorit sazku
- ziskat sazky podle ticket id
- smazat sazku podle id

## Ticket

K ticketum se muzeme chovat tak,
ze uzivatel bude mit vzdy jeden otevrenej (at uz prazdnej, nebo nejak naplnenej).
To znamena,.

Potrebuju umet:
- submitnout ticket podle user id
- ziskat current ticket podle user id
- ziskat tickety podle user id





