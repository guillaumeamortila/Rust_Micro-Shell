# TP Rust : Micro-Shell

### IABD2 - Groupe 13 :

* Guillaume AMORTILA
* Alexis LIBERGE

Notre repo : https://github.com/guillaumeamortila/Rust_Micro-Shell

##  Partie 1. Questions : Rappels de Rust, généralités

#### 1. En Rust à quoi servent les références ?


> Les références en Rust permettent d’emprunter des valeurs à des variables grâce à l'opérateur & (et &mut en cas de référence à une variable mutable). Cela est propre au  système d'ownership de Rust.


#### 2. Citez en Rust les grandes façons de déclarer ses propres types.

> Pour déclarer ses propres types en Rust on utilise Enum ou Struct.


#### 3. Rust est compilé nativement (assembleur sous forme de code machine) ou compte sur une machine virtuelle pour s’exécuter ?

> Rust est compilé nativement avec LLVM.


#### 4. Imaginons qu’on a un système avec un processeur 8bits, quelle est la valeur maximale adressable ? Écrire la solution en notation hexadécimale et décimale.

> La valeur maximum adressable d’un système avec un processeur de 8 bits est
> en décimal : 255 (2^8 - 1)
> en hexadécimal : FF


#### 5. Donnez votre définition d’un processus, citez vos sources !

> Un processus fonctionne de façon isolée, il dispose de son propre espace et n'est relié aux autres que par l'OS, ce qui lui permet de s'exécuter simultanément à d'autres.

> Concrètement, c'est un programme en cours d'exécution par un ordinateur. il peut être défini comme :
> * Un ensemble d'instructions à exécuter, pouvant être dans la mémoire morte, mais le plus souvent chargé depuis la mémoire de masse vers la mémoire vive.
> * Un espace d'adressage en mémoire vive pour stocker la pile, les données de travail, etc. 
> * Des ressources permettant des entrées-sorties de données, comme des ports réseau.
> L'exécution d'un processus dure un certain temps, avec un début et (parfois) une fin. 
> Un processus peut être démarré par un utilisateur par l'intermédiaire d'un périphérique ou bien par un autre processus.


> Source : Cours de Rust + Wikipédia



## Partie 2. Pratique - micro-shell

#### 1. Comment compiler puis exécuter son programme ? Exécuter les test ? Où sont rangés les binaires (en mode debug) ?
> La commande pour compiler son programme est ```cargo build```, et pour le lancer, ```cargo run```.
> Pour executer les tests on utilise la commande ```cargo test```.
> Les binaires de débug sont dans le dossier target/debug/build.



## Partie 3. Execution d’un Processus

#### 4. Afficher le statut d’une commande, pourquoi Rust vous force à récupérer le statut ?
> Rust doit attendre que le dernier processus de la commande soit terminé avec succès pour lancer le suivant. On parle de processus parent et enfant, le parent attendant que son processus enfant soit terminé.

#### 5. Que fait votre programme pendant que son enfant s’exécute ?
> Il attend, sagement.



## Partie 4 : Redirections - pipe my programs’

#### 7. Donnez avec vos mot une définition d’un tube entre deux programmes. Citez vos sources.
> C'est un canal de communication d'un processus à un autre. Ainsi, la sortie d'un processus alimente via ce canal l'entrée du suivant.

> sources : notre savoir + quelques pages web pour vérifier...



## Partie 5 : Executions en concurrence : Gérer des commandes en fond

#### 10. C’est quoi un processus id ? Citez vos sources.
> Un PID (Processus ID) est l'identifiant d'un processus. Chaque processus en possède un, ce qui les rend unique et différentiables. Le processus peut ainsi être identifié grâce à son PID, lors des commandes, des tâches exécutées sur une machine.

#### 12. Écrire une structure pour stocker les programmes en cours d’exécution de votre shell.
```rust
pub Struct Processus {
	pid: usize,
	status: enum,
	child: Vec<Processus>,
	parent: Vec<Processus>
}
```




**Merci pour votre lecture.**


Pour information, nous nous sommes inspiré de l'exemple de ce site https://www.joshmcguigan.com/blog/build-your-own-shell-rust/ pour construire le pipe. Ce fut simplement une béquille, (presque) pas de copier-coller, du moins aucun que nous n'avions pas compris au préalable.


Bon courage pour les corrections !

Alexis LIBERGE, Guillaume AMORTILA
4IABD 2 - Groupe 13 (MyGES)
