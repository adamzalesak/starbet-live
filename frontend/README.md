Custom farby su v assets/styles/styles.css, treba na ne porobit classy podla backgroundcolor, font color atd.. 
Ak by nejake chybali tak tam len zrob classu a potom sa to da normalne pouzivat ako tailwind
Taktiez su tam aj nejake custom styles co treba

Pre kazdy folder v componente musis urobit mod.rs, aby sa to dalo vypropagovat hore a aby to main videl

Pridal som aj normalize.css aby vsetky prehliadace mali rovnake css settings

na reponzivitu budeme asi pouzivat md: -> 768px, alebo podla potreby

Zatial je to len tak na oko, aby to ako tak fungovalo a mohli potom robit funkcionalitu a nezameriavat sa na blbosti

Budeme musiet vyriesit nejake PrivateRoutes, ktore budu pristupne iba prihlasenym uzivatelom, napr profil atd