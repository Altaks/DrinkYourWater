# DrinkYourWater 🚰

Un bot Discord intelligent qui vous rappelle de boire de l'eau à intervalles réguliers pour maintenir une bonne hydratation.

## 🌟 Fonctionnalités

- **Système de rappels d'hydratation** : Rappels personnalisés pour boire de l'eau
- **Fréquences multiples** : Choisissez entre 30 minutes, 1 heure ou 3 heures
- **Stockage persistant** : Vos rappels sont sauvegardés dans une base de données SQLite
- **Chargement automatique** : Le bot restaure vos rappels existants au redémarrage
- **Messages privés** : Rappels personnalisés envoyés par message privé Discord
- **Interface intuitive** : Commandes slash simples et faciles à utiliser

## 🚀 Installation et configuration

### Prérequis

- [Rust](https://rustup.rs/) (version 1.70+)
- Un bot Discord avec les permissions appropriées
- Un serveur Discord (guild)

### Configuration

1. **Variables d'environnement** : Créez un fichier `.env` à la racine du projet :

```env
DISCORD_BOT_TOKEN=votre_token_bot_discord
DISCORD_GUILD_ID=votre_id_serveur
```

2. **Construction et exécution** :

```bash
# Vérifier le code
just check

# Construire le projet
just build

# Lancer le bot
just run
```

## 📋 Commandes disponibles

### `/register`
Enregistrez-vous (ou un autre utilisateur) pour recevoir des rappels d'hydratation.

**Options :**
- `target` (optionnel) : L'utilisateur à enregistrer. Si non spécifié, enregistre l'utilisateur de la commande.

**Utilisation :**
1. Exécutez `/register` ou `/register @utilisateur`
2. Choisissez votre fréquence de rappel préférée (30min, 1h, ou 3h)
3. Vous commencerez à recevoir des rappels à l'intervalle sélectionné

### `/unregister`
Désinscrivez-vous des rappels d'hydratation.

**Utilisation :**
- Exécutez `/unregister` pour arrêter de recevoir des rappels

## 🗄️ Base de données

Le bot utilise SQLite pour stocker de manière persistante les données des rappels utilisateur :

- **Fichier** : `database.sqlite` (créé automatiquement dans le répertoire du bot)
- **Table** : `users`
  - `user_id` : ID utilisateur Discord (clé primaire)
  - `username` : Nom d'utilisateur Discord
  - `reminder_frequency` : Fréquence de rappel (ThirtyMin, OneHour, ThreeHours)
  - `last_reminded` : Horodatage du dernier rappel
  - `created_at` : Horodatage de l'enregistrement de l'utilisateur

## 🛠️ Développement

### Commandes de développement

```bash
# Vérifier le code
just check

# Construire le projet
just build

# Lancer le bot
just run

# Linting et formatage
just lint

# Nettoyer les fichiers de build
just clean
```

## 🔧 Détails techniques

- **Framework** : [Serenity](https://github.com/serenity-rs/serenity) (wrapper Discord API)
- **Base de données** : SQLite avec rusqlite
- **Runtime asynchrone** : Tokio
- **Système de logging** : Tracing
- **Gestion d'erreurs** : Anyhow + ThisError
- **Planification des tâches** : tokio-schedule

## 💧 Messages de rappel

Le bot envoie différents messages selon la fréquence de rappel, par exemple :
- **30 minutes** : "💧 C'est l'heure de boire un peu d'eau ! 💧"
- **1 heure** : "💧 C'est l'heure de boire un verre d'eau ! 💧"
- **3 heures** : "💧 C'est l'heure de boire une grande quantité d'eau ! 💧"

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :

- Signaler des bugs
- Proposer de nouvelles fonctionnalités
- Soumettre des pull requests

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

*Restez hydratés et en bonne santé ! 💧✨*
