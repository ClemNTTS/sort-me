### Objectif du MVP

Créer une première version fonctionnelle permettant de :

- connecter une boîte mail ;
- récupérer des emails ;
- créer des catégories personnalisées ;
- créer des règles de tri en langage naturel ;
- attribuer un coefficient d’importance de 1 à 5 à chaque règle ;
- lancer un tri manuel des emails ;
- afficher la catégorie choisie, le score de confiance et l’explication du classement.

### Hors périmètre du MVP

Ces fonctionnalités ne sont pas prioritaires pour la première version :

- multi-utilisateur ;
- multi-comptes mail ;
- application automatique de labels dans la boîte mail ;
- déplacement automatique des emails ;
- OAuth Gmail / Outlook ;
- système de paiement ;
- vrai CRM complet ;
- statistiques avancées ;
- apprentissage automatique à partir des corrections ;
- gestion fine des permissions.

### Stack technique choisie

- **Backend** : Rust
- **Framework API** : Axum
- **Runtime async** : Tokio
- **Base de données** : PostgreSQL
- **Accès DB** : SQLx
- **Frontend** : React / Next.js
- **LLM local** : Ollama
- **LLM cloud** : Mistral / Gemini
- **Conteneurisation** : Docker Compose
