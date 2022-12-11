# Rush de Noel a 42!

## L'equipe

Ce programme a ete realise par Dorian (dburgun) et moi (tjolivea) en un week-end, le langage<br>
que nous avons choisi pour realiser ce rush est Rust.

## Objectif

Le but de ce rush est de prendre un pays compose de `x` regions ayant chacun leur PIB propre,<br>
pour les fusionner jusqu'a ce qu'il n'en reste plus que `n`.

#### A noter:

- La solution trouvee doit etre la meilleure, c'est a dire fusionner en essayant de reduire<br>
au maximum les differences de PIB. Le but est donc de trouver la solution ou l'ecart-type du<br>
PIB entre les regions est le plus bas possible.
- Deux regions doivent se toucher pour etre fusionnees.
- Le nom des regions d'origine avec leur PIB et leurs regions limitrophes sont ecrits dans un<br>
fichier d'entree passe en argument au programme
- En cas d'erreur, le fichier de sortie passe en argument doit etre rempli par une erreur.
- En cas de succes la liste finale de regions doit etre mise dans le fichier de sortie passe<br>
en argument.

## Compilation et lancement

#### Librairies/Programmes requis

Rustup doit etre installe sur le systeme de compilation

#### Compilation

Executer `cargo build -r` a la racine du projet, le programme compile se situera alors dans le<br>
dossier `/target/release` sous le nom de `rush-nowel`

#### Lancement

Apres avoir compile le programme, lancer le programme comme ceci:<br>
`rush-nowel <fichier_entree> <fichier_sortie> <nombre_de_regions_souhaite>`
