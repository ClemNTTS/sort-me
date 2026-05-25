# Roadmap

## Objectif du projet

Créer une application open source auto-hébergeable permettant de trier automatiquement des emails grâce à des catégories et des règles écrites en langage naturel.

L’application doit pouvoir fonctionner avec un LLM local ou un provider cloud.

---

## Phase 0 — Cadrage du MVP

- [x] Définir le nom du projet
- [x] Écrire la phrase produit
- [x] Définir le périmètre du MVP
- [x] Choisir la stack technique
- [x] Créer le repository GitHub
- [x] Ajouter un `README.md`
- [x] Ajouter un `.env.example`
- [x] Ajouter une première version du `docker-compose.yml`

### MVP visé

- Connexion à une boîte mail
- Création de catégories
- Création de règles en langage naturel
- Attribution d’un coefficient d’importance de 1 à 5
- Classification manuelle des emails
- Affichage du résultat avec score et explication

---

## Phase 1 — Backend de base

- [x] Initialiser le backend Rust
- [x] Mettre en place Axum
- [x] Mettre en place Tokio
- [x] Configurer PostgreSQL
- [x] Ajouter SQLx
- [x] Ajouter les migrations
- [x] Créer un endpoint `GET /health`
- [x] Mettre en place la configuration via variables d’environnement
- [x] Mettre en place les logs avec `tracing`

### Modules backend

- [x] `config`
- [x] `api`
- [x] `db`
- [x] `mail`
- [x] `classifier`
- [x] `llm`
- [x] `scheduler`
- [x] `security`

---

## Phase 2 — Modèle de données

- [ ] Créer la table `mail_accounts`
- [ ] Créer la table `emails`
- [ ] Créer la table `categories`
- [ ] Créer la table `rules`
- [ ] Créer la table `classifications`
- [ ] Créer la table `settings`
- [ ] Créer la table `llm_configs`

### Entités principales

- [ ] Compte mail
- [ ] Email
- [ ] Catégorie
- [ ] Règle
- [ ] Résultat de classification
- [ ] Configuration LLM
- [ ] Paramètres de scan

---

## Phase 3 — Catégories et règles

- [ ] Créer une catégorie
- [ ] Lister les catégories
- [ ] Modifier une catégorie
- [ ] Supprimer une catégorie
- [ ] Créer une règle
- [ ] Lier une règle à une catégorie
- [ ] Définir un coefficient d’importance de 1 à 5
- [ ] Lister les règles
- [ ] Modifier une règle
- [ ] Supprimer une règle

### API prévue

- [ ] `GET /categories`
- [ ] `POST /categories`
- [ ] `PATCH /categories/:id`
- [ ] `DELETE /categories/:id`
- [ ] `GET /rules`
- [ ] `POST /rules`
- [ ] `PATCH /rules/:id`
- [ ] `DELETE /rules/:id`

---

## Phase 4 — Moteur de classification

- [ ] Définir le format d’entrée d’un email
- [ ] Définir le format de sortie attendu du LLM
- [ ] Créer le `PromptBuilder`
- [ ] Créer le `ClassificationEngine`
- [ ] Parser la réponse JSON du LLM
- [ ] Calculer le score final
- [ ] Gérer le niveau de confiance
- [ ] Gérer les cas ambigus
- [ ] Retourner une explication compréhensible

### Format de sortie cible

```json
{
  "category_id": "uuid",
  "matched_rule_ids": ["uuid"],
  "confidence": 0.87,
  "importance_score": 5,
  "reason": "Le mail exprime une intention claire d'achat."
}
```

### API prévue

- [ ] `POST /classify/test`
- [ ] `POST /classify/email/:id`

---

## Phase 5 — Providers LLM

- [ ] Créer une interface commune `LlmProvider`
- [ ] Créer un provider fake pour les tests
- [ ] Ajouter le provider Ollama
- [ ] Ajouter le provider Mistral
- [ ] Ajouter le provider Gemini
- [ ] Gérer les erreurs de provider
- [ ] Gérer les timeouts
- [ ] Gérer les réponses invalides
- [ ] Configurer le provider via `.env`

### Providers prévus

- [ ] `fake`
- [ ] `ollama`
- [ ] `mistral`
- [ ] `gemini`

---

## Phase 6 — Connexion mail

- [ ] Ajouter la configuration IMAP
- [ ] Tester la connexion à une boîte mail
- [ ] Récupérer les derniers emails
- [ ] Parser les emails
- [ ] Nettoyer le contenu HTML
- [ ] Extraire le sujet
- [ ] Extraire l’expéditeur
- [ ] Extraire la date
- [ ] Extraire un aperçu du contenu
- [ ] Stocker les emails en base
- [ ] Éviter les doublons via `message_id`

### API prévue

- [ ] `POST /mail-accounts`
- [ ] `GET /mail-accounts`
- [ ] `POST /mail-accounts/:id/test`
- [ ] `POST /emails/fetch`
- [ ] `GET /emails`

---

## Phase 7 — Tri manuel

- [ ] Ajouter un bouton logique “Trier maintenant”
- [ ] Récupérer les emails non classés
- [ ] Classer les emails un par un
- [ ] Sauvegarder les résultats
- [ ] Afficher la catégorie choisie
- [ ] Afficher le score de confiance
- [ ] Afficher la règle détectée
- [ ] Afficher l’explication
- [ ] Permettre une correction manuelle

### API prévue

- [ ] `POST /classify/run`
- [ ] `GET /classifications`
- [ ] `PATCH /classifications/:id`

---

## Phase 8 — Frontend MVP

- [ ] Initialiser le frontend
- [ ] Créer la page de connexion mail
- [ ] Créer la page centrale des emails
- [ ] Créer la page des catégories
- [ ] Créer la page des règles
- [ ] Créer la page paramètres
- [ ] Ajouter la sélection du provider LLM
- [ ] Ajouter la configuration de fréquence
- [ ] Ajouter l’affichage des résultats de classification
- [ ] Ajouter l’explication “Pourquoi ce mail est ici ?”

### Pages prévues

- [ ] `/`
- [ ] `/mail-account`
- [ ] `/emails`
- [ ] `/categories`
- [ ] `/rules`
- [ ] `/settings`

---

## Phase 9 — Scheduler

- [ ] Ajouter un worker Rust
- [ ] Lancer le worker séparément de l’API
- [ ] Ajouter la commande `api`
- [ ] Ajouter la commande `worker`
- [ ] Lire la fréquence depuis les paramètres
- [ ] Scanner les emails automatiquement
- [ ] Classer uniquement les emails non triés
- [ ] Gérer les erreurs sans arrêter le worker
- [ ] Ajouter des logs de scan

### Fréquences prévues

- [ ] Manuel
- [ ] Toutes les 5 minutes
- [ ] Toutes les 10 minutes
- [ ] Toutes les heures
- [ ] Tous les jours
- [ ] Toutes les semaines

---

## Phase 10 — Docker

- [ ] Créer le Dockerfile backend
- [ ] Créer le Dockerfile frontend
- [ ] Créer le service PostgreSQL
- [ ] Ajouter le service Ollama optionnel
- [ ] Ajouter le service worker
- [ ] Ajouter les volumes nécessaires
- [ ] Ajouter les variables d’environnement
- [ ] Tester le lancement complet avec Docker Compose

### Services prévus

- [ ] `frontend`
- [ ] `backend`
- [ ] `worker`
- [ ] `postgres`
- [ ] `ollama`

---

## Phase 11 — Sécurité

- [ ] Ne jamais exposer les clés API au frontend
- [ ] Ne jamais logger le contenu complet des emails
- [ ] Ajouter le chiffrement des identifiants mail
- [ ] Ajouter le chiffrement des clés API
- [ ] Ajouter une option “mode local uniquement”
- [ ] Documenter les données envoyées aux providers cloud
- [ ] Ajouter des avertissements dans le README
- [ ] Ajouter une rotation possible des secrets

---

## Phase 12 — Tests

- [ ] Tester les repositories SQLx
- [ ] Tester la création de catégories
- [ ] Tester la création de règles
- [ ] Tester le parsing JSON du LLM
- [ ] Tester le moteur de scoring
- [ ] Tester le provider fake
- [ ] Tester le nettoyage des emails
- [ ] Tester la classification d’un email fictif
- [ ] Tester les erreurs de provider
- [ ] Tester les migrations

---

## Phase 13 — Documentation

- [ ] Compléter le README
- [ ] Ajouter une section installation
- [ ] Ajouter une section configuration
- [ ] Ajouter une section providers LLM
- [ ] Ajouter une section sécurité
- [ ] Ajouter une section roadmap
- [ ] Ajouter des captures d’écran
- [ ] Ajouter des exemples de règles
- [ ] Ajouter une licence open source
- [ ] Ajouter un guide de contribution

---

## Phase 14 — Améliorations post-MVP

- [ ] Appliquer automatiquement des labels dans la boîte mail
- [ ] Déplacer les emails dans des dossiers
- [ ] Ajouter le support OAuth Gmail
- [ ] Ajouter le support OAuth Outlook
- [ ] Ajouter le multi-comptes mail
- [ ] Ajouter le multi-utilisateur
- [ ] Ajouter un historique complet des décisions
- [ ] Ajouter un mode dry-run
- [ ] Ajouter des statistiques de tri
- [ ] Ajouter une file de jobs avec Redis
- [ ] Ajouter une interface d’évaluation des classifications
- [ ] Ajouter un système d’apprentissage à partir des corrections manuelles
- [ ] Ajouter l’import/export des règles
- [ ] Ajouter des templates de règles par métier
