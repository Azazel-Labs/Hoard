## Erreurs du serveur

error-account-not-found = compte introuvable
error-asset-gone = ressource introuvable
error-asset-not-found = ressource { $id } introuvable
error-blob-not-found = blob { $hash } introuvable
error-collection-not-found = collection { $id } introuvable
error-connection-not-found = connexion introuvable
error-derivative-not-found = dérivé { $kind } de { $id } introuvable
error-file-in-pack-not-found = { $path } introuvable dans le pack { $id }
error-site-not-a-page = seules les pages web s’ouvrent comme des pages
error-site-link-expired = l’adresse de cette page a expiré ; rouvrez la page depuis Hoard
error-format-not-found = format { $file } introuvable
error-frame-sheet-aseprite-only = seuls les fichiers Aseprite ont une planche d’images
error-frame-sheet-frames = ce fichier n’a qu’une image, ou trop pour les disposer
error-location-not-found = emplacement { $id } introuvable
error-maintenance-task-not-found = tâche de maintenance { $id } introuvable
error-open-upload-not-found = envoi en cours { $id } introuvable
error-original-not-found = fichier d’origine de { $id } introuvable
error-pack-not-found = pack { $id } introuvable
error-preview-too-big = cette image est trop grande pour en faire un aperçu léger
error-project-not-found = projet { $name } introuvable
error-saved-search-not-found = recherche enregistrée { $id } introuvable
error-sign-in-first = connectez-vous d’abord (ou envoyez un jeton d’accès)
error-those-assets-not-found = ressources introuvables
error-thumbnail-not-found = miniature de { $id } introuvable
error-token-not-found = jeton introuvable
error-upload-not-found = envoi { $id } introuvable
error-pack-needs-name = un pack doit avoir un nom
error-no-library = il n’y a encore aucun emplacement de bibliothèque où stocker les fichiers : ajoutez-en un géré dans Paramètres → Emplacements de la bibliothèque
error-upload-failed = échec de l’envoi : { $reason }
error-upload-no-files = aucun fichier dans l’envoi
error-analyze-nothing-to-do = rien à faire : rules=false et le LLM est désactivé
error-too-many-assets = trop de ressources dans une seule requête
error-change-not-found = cette modification n'est pas dans l'historique de cet asset
error-bulk-by-look = une recherche par apparence ne peut pas être modifiée d'un coup : sélectionnez plutôt les assets
error-too-many-items = trop d’éléments dans une seule requête
error-not-a-hash = ce n’est pas un hachage blake3 en hexadécimal
error-merge-into-tag = into doit être une étiquette
# A tag asked about isn't one: $text is what was given.
error-not-a-tag = « { $text } » n'est pas une étiquette
# Putting a tag under a broader one would go round in a circle: $parent is already under $tag (or is it).
error-tag-parent-loop = { $parent } est déjà sous { $tag }, donc { $tag } ne peut pas passer dessous
error-nothing-to-merge = rien à fusionner
error-selection-size = indiquez entre 1 et { $max } identifiants de ressources
error-collection-needs-name = une collection doit avoir un nom
error-collection-exists = il existe déjà une collection nommée { $name }
error-collection-empty = la collection { $name } est vide
error-conversion-failed = impossible de convertir en { $format } : { $reason }
error-max-size = max doit être une puissance de deux entre 64 et 16384
error-replace-ext = ext doit être une courte extension de fichier
error-empty-file = fichier vide
error-imitation-silent = Rien dans l’enregistrement ne ressort du bruit de fond. Réessayez plus près du micro.
error-replace-duplicate = le fichier converti fait doublon avec un fichier déjà présent dans le pack ; le paquet y a été fusionné
error-ai-address = L’adresse de l’IA doit commencer par http:// ou https://
error-hosted-key = Saisissez la clé d’API de { $provider }
error-hosted-address = L’adresse du service de recherche d’images doit commencer par http:// ou https://
error-local-llm-action = action vaut list, download ou delete, et ces deux dernières demandent un modèle
error-role = Un rôle vaut view, edit ou admin
error-admin-only = seul un administrateur peut faire cela
error-view-only = ce compte peut consulter mais rien modifier : un administrateur peut changer cela dans Paramètres → Compte
error-portal-only = connectez-vous au portail pour gérer les comptes et les jetons
error-not-from-portal = cette requête ne provient pas du portail Hoard
error-account-name = Choisissez un nom de 64 caractères au plus
error-password-length = Utilisez un mot de passe d’au moins { $min } caractères
error-too-many-codes = Trop de codes erronés : patientez { $seconds } secondes
error-already-set-up = Un compte existe déjà : connectez-vous
error-too-many-tries = Trop de tentatives erronées : patientez { $seconds } secondes
error-directory-unavailable = L’annuaire n’a pas pu vérifier votre mot de passe : { $reason }
error-password-elsewhere = Votre mot de passe est géré par l’annuaire ou le fournisseur avec lequel vous vous connectez
error-current-password = Votre mot de passe actuel est incorrect
error-account-source = Un compte se connecte avec Hoard (internal), un annuaire (ldap) ou OAuth
error-account-exists = Il existe déjà un compte portant ce nom
error-own-role = Seul un autre administrateur peut changer votre rôle
error-remove-self = Vous ne pouvez pas supprimer votre propre compte pendant que vous y êtes connecté
error-token-name = Nommez le jeton d’après ce qui l’utilisera, en 80 caractères au plus
error-token-role = Un jeton ne peut pas faire plus que le compte qui le crée
error-wrong-setup-code = Ce n’est pas le code de configuration donné au serveur dans HOARD_SETUP_CODE
error-wrong-password = Ce nom et ce mot de passe ne correspondent pas
error-folder-missing = { $path } est introuvable (un disque non monté ?)
error-s3-path = un emplacement S3 s’écrit s3://bucket
error-s3-address = l’adresse S3 doit commencer par http:// ou https://
error-s3-header = un en-tête S3 supplémentaire s’écrit Name=value, et « { $header } » n’en est pas un
error-keychain = impossible de l’enregistrer dans le trousseau du système : { $reason }
error-not-built = cette copie de Hoard a été compilée sans { $feature }
error-openai-auth-unreachable = impossible de joindre auth.openai.com
error-openai-unexpected = réponse inattendue d’OpenAI
error-openai-answered = OpenAI a répondu { $status } : { $reason }
error-chatgpt-id-token-not-jwt = le jeton d’identité n’est pas un JWT
error-chatgpt-id-token-issuer = le jeton d’identité provient de { $issuer }, et non de { $expected }
error-chatgpt-id-token-audience = le jeton d’identité est destiné à une autre application
error-chatgpt-id-token-expired = le jeton d’identité a expiré
error-chatgpt-id-token-nonce = le jeton d’identité n’appartient pas à cette connexion
error-chatgpt-no-state = l’adresse n’a pas de state ; copiez l’adresse entière
error-chatgpt-not-started = cette connexion n’a pas été lancée ici, ou elle l’a été il y a plus de 15 minutes. Relancez-la depuis les Paramètres.
error-chatgpt-canceled = la connexion a été annulée
error-chatgpt-refused = ChatGPT a refusé la connexion : { $reason }
error-chatgpt-no-code = l’adresse ne contient pas de code de connexion ; copiez l’adresse entière
error-chatgpt-other-registration = ChatGPT a répondu pour un autre enregistrement de ce serveur ; relancez la connexion
error-chatgpt-other-registration-desktop = ChatGPT a répondu pour une autre copie de Hoard ; relancez la connexion
error-chatgpt-not-registered = ChatGPT n’a pas terminé l’enregistrement de Hoard ; relancez la connexion
error-chatgpt-code-used = le code de connexion a déjà été utilisé ou a expiré ; reconnectez-vous
error-chatgpt-refused-code = ChatGPT a refusé la connexion ({ $code })
error-chatgpt-no-id-token = ChatGPT n’a envoyé aucun jeton d’identité
error-chatgpt-plan-not-allowed = Connecté, mais l’utilisation de l’abonnement n’a pas été autorisée. Reconnectez-vous et autorisez-la, ou utilisez plutôt une clé d’API.
error-chatgpt-problem = ChatGPT : { $problem }
error-chatgpt-not-signed-in = non connecté à ChatGPT ; connectez-vous depuis Paramètres → Étiquetage par IA
error-chatgpt-refresh-later = ChatGPT n’a pas pu renouveler la connexion pour l’instant ({ $code }) ; nouvel essai plus tard
error-chatgpt-signed-out-problem = ChatGPT a déconnecté ce serveur ({ $code }). Reconnectez-vous pour continuer l’étiquetage avec votre abonnement.
error-chatgpt-signed-out-problem-desktop = ChatGPT a déconnecté Hoard ({ $code }). Reconnectez-vous pour continuer l’étiquetage avec votre abonnement.
error-chatgpt-signed-out = ChatGPT a déconnecté ce serveur ({ $code }) ; reconnectez-vous depuis Paramètres → Étiquetage par IA
error-chatgpt-signed-out-desktop = ChatGPT a déconnecté Hoard ({ $code }) ; reconnectez-vous depuis Paramètres → Étiquetage par IA
error-openai-api-unreachable = impossible de joindre api.openai.com
error-openai-broke-off = la réponse d’OpenAI s’est interrompue
error-chatgpt-kept-refusing = OpenAI a refusé à plusieurs reprises la connexion ChatGPT ; reconnectez-vous depuis Paramètres → Étiquetage par IA
error-chatgpt-request-failed = la requête a échoué
error-chatgpt-cut-off = la réponse a été tronquée ({ $reason })
error-chatgpt-cut-off-no-reason = la réponse a été tronquée (sans raison indiquée)
error-openai-ended-early = la réponse d’OpenAI s’est arrêtée avant d’être complète
error-chatgpt-usage-limit = l’utilisation de votre abonnement ChatGPT pour Hoard est épuisée pour le moment. Consultez ou modifiez la limite sur { $url }
error-chatgpt-not-eligible = ce compte ou cet espace de travail ChatGPT ne peut pas utiliser son abonnement dans d’autres applications. Utilisez plutôt une clé d’API.
error-chatgpt-usage-unavailable = ChatGPT n’a pas pu vérifier l’utilisation de l’abonnement pour l’instant ; nouvel essai plus tard
error-chatgpt-unsupported = l’utilisation via l’abonnement ChatGPT ne prend pas en charge { $param } avec ce modèle
error-chatgpt-unsupported-images = l’utilisation via l’abonnement ChatGPT ne prend pas en charge { $param } avec ce modèle ; choisissez un modèle qui accepte les images, ou désactivez l’envoi des miniatures
error-chatgpt-unsupported-part = l’utilisation via l’abonnement ChatGPT ne prend pas en charge une partie de la requête avec ce modèle
error-chatgpt-port = port est celui par lequel le navigateur joint Hoard
error-chatgpt-paste-address = collez l’adresse entière, qui commence par http://127.0.0.1
error-too-many-hashes = au plus { $max } hachages à la fois
error-credits-pack = pack { $key }
error-credits-collection = collection { $key }
error-format-too-long = le format est trop long
error-not-started = le serveur n’a pas démarré
error-not-started-desktop = Hoard n’a pas démarré
error-not-format-file = ce n’est pas un fichier de format : { $name }
error-format-name = le nom doit contenir une lettre ou un chiffre de A à Z ou de 0 à 9
error-format-miss-name = les fichiers du format ne prennent pas { $file }
error-format-miss-store = le format ne prend pas { $file } pour ceux de la boutique (voir require)
error-format-miss-title = le format ne trouve aucun titre dans { $file }
error-cant-download-as = les fichiers .{ $ext } ne peuvent pas être téléchargés en { $format }
error-only-images-resize = seules les images peuvent être redimensionnées
error-https-no-certificate = Collez ou envoyez d’abord votre certificat et sa clé
error-https-no-cert-found = Aucun certificat trouvé : collez le texte qui commence par -----BEGIN CERTIFICATE-----
error-https-no-key-found = Aucune clé privée trouvée : collez le texte qui commence par -----BEGIN PRIVATE KEY-----
error-https-several-keys = Il y a plusieurs clés privées : collez seulement celle qui va avec le certificat
error-https-key-password = La clé est protégée par un mot de passe : fournissez-la sans (openssl pkey -in key.pem -out plain.pem)
error-https-cert-not-pem = Le certificat n’est pas un PEM valide
error-https-key-not-pem = La clé privée n’est pas un PEM valide
error-https-key-mismatch = La clé ne correspond pas au certificat. Si vous avez fourni une chaîne, mettez le certificat de votre serveur en premier
error-https-unusable = Le certificat ou la clé est inutilisable
error-https-expired = Ce certificat a expiré le { $date }
error-https-not-yet-valid = Ce certificat n’est pas valide avant le { $date }
error-https-unreadable = Ce n’est pas un certificat que Hoard peut lire
error-https-bad-name = « { $name } » n’est pas une adresse qu’un certificat peut contenir
error-https-test-timed-out = la connexion de test a expiré
error-https-test-failed = la connexion de test a échoué
error-https-no-ca = Hoard n’a pas encore créé son autorité de certification
error-https-unavailable = HTTPS n’est pas disponible sur ce serveur
error-https-certificate-mode = Un certificat va avec « Mon certificat »
error-https-test = Le certificat n’a pas fonctionné lors d’une connexion de test : { $reason }
error-nothing-to-retry = rien ici ne peut être réessayé
error-ai-unreachable = impossible de joindre { $url }
error-ai-key-refused = { $url } a refusé la clé d’API ({ $status })
error-ai-answered = { $url } a répondu { $status } : { $reason }
error-ai-not-json = { $url }/models n’a pas répondu en JSON
error-model-server-refused = le serveur de modèles : { $reason }
error-no-model-server = aucun serveur de modèles ne répond à { $url }
error-not-llama-router = { $url } n’est pas un serveur llama.cpp en mode routeur, il ne peut donc pas télécharger de modèles
error-model-download = impossible de télécharger { $model } : { $reason }
error-model-delete = impossible de supprimer { $model } : { $reason }
error-unknown-kind = type de ressource inconnu { $kind }
error-watched-is-folder = un emplacement surveillé est un dossier sur ce serveur
error-cant-reach = impossible de joindre { $path } : { $reason }
error-folder-full-path = indiquez le chemin complet du dossier
error-cant-create = impossible de créer { $path }
error-cant-write = impossible d’écrire dans { $path }
error-cant-read = impossible de lire { $path }
error-overlaps-location = cela chevauche l’emplacement { $name } ({ $path })
error-overlaps-data = cela chevauche le dossier de données de Hoard ({ $path })
error-overlaps-cache = cela chevauche le dossier de cache de Hoard ({ $path })
error-not-watched = { $name } n’est pas un dossier surveillé
error-watched-no-kinds = un dossier surveillé n’accepte pas de nouveaux fichiers, il n’a donc pas de types à modifier
error-keep-catch-all = faites plutôt d’un autre emplacement l’emplacement par défaut
error-location-holds-files = { $name } contient encore des fichiers : déplacez-les d’abord vers les autres emplacements (POST /api/admin/locations/{ $id }/empty)
error-location-unreachable = { $name } est injoignable, Hoard ne peut donc pas savoir s’il contient encore des fichiers
error-relocate-running = des fichiers sont déjà en cours de déplacement entre emplacements ; réessayez quand ce sera terminé
error-watched-not-moved = { $name } est un dossier surveillé : Hoard ne déplace pas ses fichiers
error-no-other-location = il n’y a aucun autre emplacement de bibliothèque où déplacer les fichiers : ajoutez-en un d’abord
error-full-path = indiquez un chemin complet
error-pick-folder = choisissez un dossier
error-task-running = « { $task } » est déjà en cours
error-job-cant-stop = « { $job } » ne peut pas être arrêté une fois lancé
error-job-not-found = cette tâche n’est plus en attente ni en cours
error-task-cant-stop = « { $task } » ne peut pas être arrêté en cours de route
error-task-not-running = « { $task } » n’est pas en cours
error-token-account-gone = le compte de ce jeton n’existe plus, il n’a donc ni favoris ni recherches
error-favorites-too-many = au plus 10 000 à la fois
error-saved-search-name = une recherche enregistrée doit avoir un nom de 80 caractères au plus
error-saved-search-too-long = cette recherche est trop longue pour être enregistrée
error-hoard-seen = le trésor peut enregistrer au plus 32 types de ressources, chacun nommé en lettres minuscules
error-texture-pattern-map = Le modèle de nom des textures doit contenir { "{" }map{ "}" }, sinon toutes les cartes d’un matériau porteraient le même nom
error-texture-pattern-fields = Le modèle de nom des textures peut utiliser { "{" }name{ "}" }, { "{" }map{ "}" } et { "{" }size{ "}" }
error-texture-name-chars = Les noms de textures ne peuvent pas contenir < > : " / \ | ? ou *
error-texture-map-type = Il n’existe aucun type de carte nommé { $role }
server-role-base-color = couleur
server-role-normal = normale
server-role-normal-dx = normale (DirectX)
server-role-roughness = rugosité
server-role-gloss = brillance
server-role-metalness = métallicité
server-role-ao = occlusion ambiante
server-role-orm = ORM
server-role-mra = MRA
server-role-rma = RMA
server-role-height = hauteur
server-role-bump = relief
server-role-cavity = cavité
server-role-specular = spéculaire
server-role-opacity = opacité
server-role-emissive = émission
server-role-translucency = translucidité
error-texture-suffix-missing = Donnez un suffixe aux cartes de type { $map }
error-texture-suffix-chars = Le suffixe des cartes de type { $map } ne peut pas contenir < > : " / \ | ? ou *
error-texture-ending-missing = Chaque terminaison personnalisée doit indiquer la terminaison à rechercher
error-texture-ending-chars = Les terminaisons personnalisées ne peuvent pas contenir < > : " / \ | ? ou *
error-texture-ending-twice = { $ending } figure deux fois dans vos terminaisons
error-oauth-no-client-id = Ce lien n’a pas de client_id.
error-oauth-unknown-app = Cette application n’est pas (ou plus) enregistrée ici. Relancez la connexion depuis l’application.
error-oauth-redirect = L’application a demandé à être renvoyée vers une adresse qu’elle n’a pas enregistrée.
error-oauth-response-type = Hoard ne délivre que des codes de connexion (response_type=code).
error-oauth-pkce = L’application doit utiliser PKCE avec S256 pour se connecter à Hoard.
error-light-preview-images = seules les images ont un aperçu léger
error-no-light-preview = pas d’aperçu léger : { $reason }
error-no-frame-sheet = pas de planche d’images : { $reason }
error-thumbnail-source = source de miniature inconnue : { $source }
error-thumbnail-too-big = cette image est trop grande pour une miniature
error-thumbnail-models-only = seuls les modèles acceptent une miniature dessinée par un navigateur
error-not-a-picture = ce n’est pas une image que Hoard peut lire : { $reason }
error-project-name = un projet doit avoir un nom de 200 caractères au plus
error-project-too-big = au plus un million de ressources par projet
error-project-too-many-at-once = au plus 100 000 à la fois
error-server-name = Choisissez un nom de { $max } caractères au plus, sur une seule ligne
error-tool-setting = { $tool } : choisissez désactivé, automatique, ou indiquez un chemin
error-ai-model = Choisissez le modèle que le service d’IA doit utiliser
error-public-address = L’adresse publique doit commencer par http:// ou https://
error-update-hook-address = L’adresse à appeler pour les mises à jour doit commencer par http:// ou https://
error-update-hook-auth = L’en-tête Authorization ne peut pas contenir de sauts de ligne ni d’autres caractères de contrôle
error-no-soundfont = Il n’y a pas de SoundFont à { $path } sur le serveur
error-no-soundfont-desktop = Il n’y a pas de SoundFont à { $path }
error-soundfont-bad = { $name } n’est pas un SoundFont avec lequel le MIDI peut jouer : { $reason }
error-workers = Tâches de fond : entre 1 et { $max }
error-ai-batch = Ressources par requête d’IA : entre 1 et 200
error-conversion-cache = Téléchargements convertis : au moins 100 Mo
error-backup-keep = Sauvegardes planifiées : garder de 1 à { $max }
error-backup-copy-to = Sauvegardes planifiées : { $path } n’est pas un dossier sur le serveur (indiquez son chemin complet)
error-conversion-keep-days = Règles de conversion : conservez les originaux de 1 à { $max } jours
error-conversion-target = Règles de conversion : Hoard ne sait pas convertir des fichiers en { $format }
error-conversion-no-source = Règles de conversion : chaque règle doit avoir au moins un format à convertir
# $ext is a file extension, like obj; $format is what it would become, like glb.
error-conversion-source = Règles de conversion : les fichiers .{ $ext } ne peuvent pas être convertis en { $format }
error-conversion-twice = Règles de conversion : .{ $ext } figure dans deux règles
error-conversion-cant-undo = Ce fichier n’a pas été converti par une règle, ou son original a été supprimé.
error-conversion-path-taken = Un autre fichier se trouve maintenant à { $path }, l’original ne peut donc pas y revenir.
error-limits-zero = Les limites ne peuvent pas être nulles
error-settings-object = un objet JSON de paramètres était attendu
error-bad-setting = paramètre incorrect : { $reason }
error-auth-default-role = Le rôle par défaut vaut view, edit ou admin
error-auth-session-hours = Les sessions durent entre 1 heure et un an
error-auth-settings = paramètres d’authentification incorrects : { $reason }
error-ldap-settings = paramètres LDAP incorrects : { $reason }
error-ldap-no-server = Ajoutez l’adresse du serveur d’annuaire
error-ldap-base-dn = Indiquez où se trouvent les entrées des personnes (le DN de base)
error-ldap-username-attribute = Indiquez quel attribut contient le nom avec lequel on se connecte
error-ldap-bind-dn = Indiquez le compte qui recherche les personnes, ou recherchez-les de façon anonyme
error-ldap-ca-cert = L’autorité de certification n’est pas un certificat PEM (-----BEGIN CERTIFICATE-----)
error-ldap-empty-server = L’adresse d’un serveur d’annuaire est vide
error-ldap-server-form = { $server } : l’adresse d’un serveur d’annuaire s’écrit host:port, ldap://… ou ldaps://…
error-ldap-not-starttls = { $server } : c’est du LDAPS, pas du StartTLS
error-ldap-no-certificate = aucun certificat dedans
error-ldap-password-again = Saisissez à nouveau le mot de passe du compte qui recherche les personnes : celui enregistré n’est envoyé qu’aux serveurs pour lesquels il a été donné
error-ldap-unreachable = aucun serveur d’annuaire n’a pu être joint ({ $reasons })
error-ldap-no-anonymous = l’annuaire n’autorise pas la recherche anonyme de personnes ({ $reason })
error-ldap-bind-failed = le compte qui recherche les personnes n’a pas pu se connecter ({ $reason })
error-ldap-search = recherche de { $name } sous { $base }
error-ldap-group-search = recherche des groupes de { $name } sous { $base }
error-ldap-several = { $count } entrées de l’annuaire correspondent à { $name } : affinez le filtre d’utilisateurs
error-ldap-password-check = l’annuaire a refusé de vérifier le mot de passe de { $name } ({ $reason })
error-ldap-wrong-password = L’annuaire ne connaît pas { $name } avec ce mot de passe
error-oauth-client-id = Indiquez l’ID client que le fournisseur a créé pour Hoard
error-oauth-addresses = Les adresses d’autorisation et de jeton doivent commencer par https://
error-oauth-userinfo-address = L’adresse des informations utilisateur doit commencer par https://
error-oauth-username-claim = Indiquez quel claim contient le nom de la personne
error-oauth-secret-again = Saisissez à nouveau le secret client : celui enregistré n’est envoyé qu’à l’adresse de jeton pour laquelle il a été donné
error-oauth-issuer = L’émetteur doit commencer par https://
error-oauth-discovery-unreachable = Impossible de joindre { $url } : { $reason }
error-oauth-discovery-status = { $url } a répondu { $status }
error-oauth-discovery-bad = { $url } n’est pas un document de découverte
error-oauth-discovery-incomplete = { $url } n’indique pas les adresses d’autorisation et de jeton
error-signin-no-group = { $name } n’appartient à aucun groupe autorisé à utiliser Hoard
error-signin-other-source = Il existe déjà un compte Hoard nommé { $name } qui se connecte d’une autre façon
error-signin-other-subject = Il existe déjà un compte Hoard nommé { $name } pour une autre personne chez le fournisseur
error-signin-no-account = { $name } n’a pas encore de compte Hoard : un administrateur peut l’ajouter dans Paramètres → Compte
error-signin-bad-name = Ce nom ne peut pas être celui d’un compte Hoard (64 caractères au plus)
error-oauth-off = La connexion par OAuth n’est pas activée
error-oauth-bad-authorization-address = l’adresse d’autorisation n’est pas valide
error-oauth-refused = Le fournisseur ne vous a pas connecté : { $reason }
error-oauth-no-code = Le fournisseur vous a renvoyé sans connexion
error-oauth-expired = Cette connexion a pris trop de temps ou a été lancée ailleurs : réessayez
error-oauth-no-claim = Le fournisseur n’a pas indiqué qui vous êtes (pas de claim { $claim })
error-oauth-no-sub = Le fournisseur n’a pas indiqué qui vous êtes (pas de claim sub ni id)
error-oauth-token-unreachable = impossible de joindre l’adresse de jeton du fournisseur
error-oauth-token-refused = Le fournisseur a refusé de terminer la connexion : { $reason }
error-oauth-no-access-token = le fournisseur n’a envoyé aucun jeton d’accès
error-oauth-userinfo-unreachable = impossible de joindre l’adresse des informations utilisateur du fournisseur
error-oauth-userinfo-refused = Le fournisseur a refusé de dire qui s’est connecté ({ $status })
error-oauth-userinfo-not-json = les informations utilisateur du fournisseur ne sont pas en JSON
error-format-no-name = name est vide
error-format-no-files = files est vide : indiquez quels fichiers le format lit, par exemple ["*.json"]
error-format-empty-selector = vide
error-format-unopened-bracket = un crochet fermant sans crochet ouvrant
error-format-unclosed-bracket = un crochet n’est pas fermé
error-format-empty-step-dots = une étape vide (deux points d’affilée, ou un point à une extrémité)
error-format-unclosed-except = !( n’est pas fermé
error-format-unclosed-filter = [ n’est pas fermé
error-format-after-filters = { $rest } après la clé et les filtres de { $step }
error-format-except-needs-glob = !(…) se place après une clé contenant *
error-format-except-not-glob = !(…) se place après une clé contenant *, pas après { $key }
error-format-empty-step = une étape vide
error-format-not-a-filter = [{ $filter }] n’est pas un filtre : utilisez [], [key=value|other], [array], [string], [number], [bool] ou [object]
error-format-filter-needs-value = [{ $filter }] demande une clé et une valeur
error-store-search-empty = rien à rechercher ; saisissez le nom du pack
error-update-hook-not-set = définissez d’abord une adresse à appeler
error-upload-stalled = l’envoi de { $path } est bloqué ; renvoyez cette partie
error-upload-busy = { $path } est déjà en cours d’écriture
error-upload-incomplete = { $count } fichiers sont incomplets, par exemple { $files }
error-upload-no-files-listed = aucun fichier
error-upload-not-part = { $path } ne fait pas partie de l’envoi { $id }, ou l’envoi est clos
error-bad-path = chemin incorrect
error-upload-part-offset = { $offset } n’est pas le début d’une partie de { $path }
error-upload-interrupted = envoi interrompu : { $reason }
error-upload-part-long = la partie { $part } de { $path } dépasse { $len } octets
error-upload-part-short = partie { $part } de { $path } : { $got } octets reçus, { $len } attendus
error-upload-file-long = { $path } est plus grand que déclaré
error-upload-file-short = { $path } : { $got } octets reçus, { $len } attendus
error-upload-complete = l’envoi est déjà terminé
error-already-imported = déjà importé sous le nom « { $name } »

## Textes du serveur

server-store-place-readme = README
server-store-place-folders = dossiers
server-store-why-linked = Lien trouvé dans le pack ({ $place })
server-store-why-same-title = Même titre que dans le pack ({ $place })
server-store-why-by = Par { $author }, nommé dans le pack ({ $place })
server-store-why-meshes = À peu près autant de maillages que le pack ({ $count })
server-store-why-textures = À peu près autant de textures que le pack ({ $count })
server-chatgpt-revoke-failed = Déconnecté ici, mais OpenAI n’a pas pu être prévenu. Pour en être sûr, déconnectez Hoard dans les paramètres de ChatGPT → Sécurité et connexion → Se connecter avec ChatGPT.
server-chatgpt-signed-in-title = Connecté à ChatGPT
server-chatgpt-signed-in = Hoard est connecté. L’étiquetage par IA peut désormais utiliser votre abonnement ChatGPT.
server-chatgpt-signed-in-as = Hoard est connecté en tant que { $email }. L’étiquetage par IA peut désormais utiliser votre abonnement ChatGPT.
server-chatgpt-not-finished-title = Connexion non terminée
server-chatgpt-other-computer = Si vous avez lancé la connexion depuis Hoard sur un autre ordinateur, copiez l’adresse de cette page depuis la barre d’adresse et collez-la dans les Paramètres de Hoard sur cet ordinateur.
server-back-to-settings = Retour aux Paramètres
server-issues-pack = Packs qui n’ont pas pu être ajoutés
server-issues-pack-blurb = Supprimez le pack et ajoutez-le à nouveau, ou ouvrez-le pour voir ce qu’il contient.
server-issues-damaged = Fichiers stockés endommagés ou disparus
server-issues-damaged-blurb = Trouvés par « Vérifier l’intégrité des fichiers ». Restaurez-les depuis une sauvegarde ou ajoutez à nouveau leur pack, puis relancez « Vérifier l’intégrité des fichiers ».
server-issues-folder = Dossiers surveillés illisibles
server-issues-folder-blurb = Leurs packs restent tels quels jusqu’à ce que le dossier soit de nouveau lisible.
server-issues-missing = Packs surveillés dont le dossier a disparu
server-issues-missing-blurb = Chaque pack est conservé 3 jours au cas où son dossier serait seulement hors ligne, puis supprimé.
server-issues-job = Tâches de fond qui ont échoué
server-issues-job-blurb = Réessayer relance la tâche depuis le début.
server-issues-file = Fichiers qui n’ont pas pu être ouverts
server-issues-file-blurb = Hoard n’a pas pu les ouvrir ou les stocker. Un pack surveillé les réessaie à sa prochaine analyse ; un fichier déjà dans la bibliothèque garde ce qu’il avait avant.
server-issues-unpack = Archives qui n’ont pas pu être décompressées
server-issues-unpack-blurb = Chacune est gardée dans son pack comme simple fichier : rien de ce qu’elle contient n’est dans la bibliothèque.
server-issues-read = Fichiers que Hoard n’a pas su interpréter
server-issues-read-blurb = Ils sont dans la bibliothèque, mais sans les informations, étiquettes ou miniature que leur lecture apporte.
server-issues-render = Modèles sur lesquels le moteur de miniatures a échoué
server-issues-render-blurb = Ils n’ont pas de miniature. Les ouvrir dans la visionneuse peut quand même fonctionner, et permet d’en enregistrer une.
server-issues-convert = Modèles que Blender n’a pas pu convertir
server-issues-convert-blurb = Ils n’ont pas de copie GLB et ne peuvent donc pas être affichés en 3D.
server-issues-unreal = Paquets Unreal qui n’ont pas pu être convertis
server-issues-unreal-blurb = La page Unreal du pack contient le reste de ses paquets.
server-issues-reformat = Fichiers qu’une règle de conversion n’a pas pu convertir
server-issues-reformat-blurb = Ils sont laissés tels quels.
server-issues-materials = Packs dont les matériaux n’ont pas pu être déterminés
server-issues-materials-blurb = Leurs textures sont toutes là ; elles ne sont simplement pas regroupées en matériaux.
server-issues-material-thumb = Matériaux sans miniature
server-issues-material-thumb-blurb = Le matériau a été trouvé, mais son aperçu n’a pas pu être dessiné.
server-issues-store-files = Fichiers de boutique illisibles
server-issues-store-files-blurb = Fichiers de la boutique d’où vient un pack, comme un JSON Megascans, qui le nomment et l’étiquettent.
server-issues-scan-again = Analyser à nouveau
server-issues-retry = Réessayer
server-issues-reanalyze = Réanalyser le pack
server-issues-convert-again = Convertir à nouveau
server-issues-look-again = Chercher à nouveau
server-issues-read-again = Relire
server-issues-blob-missing = Le fichier stocké a disparu.
server-issues-blob-damaged = Le fichier stocké ne correspond pas à ce qui a été stocké : il est endommagé.
server-issues-folder-gone = Le dossier « { $folder } » est introuvable.
server-job-a-pack = un pack
server-job-ingest = Intégration de { $pack }
server-job-retag = Réétiquetage de { $pack }
server-job-llm-tag = Étiquetage de { $pack } par IA
server-job-convert = Conversion de { $pack } avec Blender
server-job-unreal = Conversion des paquets Unreal de { $pack }
server-job-reformat = Conversion des fichiers de { $pack } selon les règles de conversion
server-job-scan = Analyse d’un dossier surveillé
server-job-relocate = Déplacement de fichiers entre emplacements de la bibliothèque
server-job-looks = Analyse des couleurs et de l’aspect
server-job-sounds = Écoute des sons
server-job-embed = Apprentissage du contenu des images
server-job-hear = Apprentissage de ce à quoi ressemblent les sons
server-job-materials = Recherche des matériaux de { $pack }
server-job-read-files = Lecture des fichiers de { $pack }
server-job-store-lookup = Recherche d’une page de boutique
server-maintenance-backup = Sauvegarder la base de données
server-maintenance-backup-blurb = Une copie cohérente de la base de données (packs, étiquettes, notes, collections et Paramètres) dans le dossier de sauvegardes à côté d’elle. Les fichiers de la bibliothèque n’y sont pas : ils ne changent jamais une fois stockés, donc n’importe quelle synchronisation de fichiers les sauvegarde.
server-maintenance-scrub = Vérifier l’intégrité des fichiers
server-maintenance-scrub-blurb = Relit chaque fichier stocké et le compare à son empreinte, pour repérer les fichiers altérés ou disparus d’un disque ou d’un NAS. Prend du temps sur une grande bibliothèque.
server-maintenance-reanalyze = Réanalyser tous les packs
server-maintenance-reanalyze-blurb = Relit chaque fichier avec cette version de Hoard : miniatures, aperçus, détails et étiquettes des règles sont refaits. Vos propres modifications restent ; l’IA n’est pas réinterrogée.
server-maintenance-clear-cache = Vider les conversions en cache
server-maintenance-clear-cache-blurb = Supprime les fichiers conservés des conversions « Télécharger en » (ils sont refaits à la prochaine demande) et ce que des travaux interrompus ont laissé.
server-maintenance-shrink-thumbs = Réduire les miniatures
server-maintenance-shrink-thumbs-blurb = Stocke les miniatures et vues de modèles conservées en PNG comme le demande Paramètres → Performances → Miniatures, ce qui les rend généralement plusieurs fois plus petites. Celles avec transparence, et le pixel art, restent en PNG.
server-maintenance-avif-thumbs = Convertir les miniatures en AVIF
server-maintenance-avif-thumbs-blurb = Convertit toutes les miniatures maintenant, plutôt qu’au fur et à mesure de leur premier affichage, quand les miniatures sont envoyées en AVIF.
server-maintenance-gc = Supprimer les fichiers inutilisés
server-maintenance-gc-blurb = Supprime les fichiers stockés que plus rien ne référence, comme les fichiers des packs supprimés définitivement.
server-update-no-address = aucune adresse à appeler n’est définie
server-update-no-answer = aucune réponse en 30 secondes

## Paramètres : Apparence

appearance-language = Langue
appearance-language-auto = Celle du navigateur ({ $name })

## Racine

root-unreachable = Impossible de joindre le serveur Hoard : { $error }
root-unreachable-desktop = Hoard ne répond pas : { $error }
root-desktop-reopen = Cette fenêtre a été déconnectée. Fermez Hoard et rouvrez-le pour continuer.
root-try-again = Réessayer
root-lazy-connect = la connexion de l’application
window-minimize = Réduire
window-maximize = Agrandir
window-restore = Restaurer
window-close = Fermer

## Application : la bibliothèque, sa barre supérieure et ses filtres

app-lazy-settings = les paramètres
app-lazy-archive = les archives
app-lazy-shortcuts = les raccourcis
app-lazy-setup = la configuration
app-theme-system = Thème du système
app-theme-light = Thème clair
app-theme-dark = Thème sombre
app-color-red = Rouge
app-color-orange = Orange
app-color-yellow = Jaune
app-color-green = Vert
app-color-teal = Bleu-vert
app-color-blue = Bleu
app-color-purple = Violet
app-color-pink = Rose
app-color-brown = Marron
app-color-sand = Sable
app-color-gray = Gris
app-color-slate = Ardoise
app-color-black = Noir
app-color-white = Blanc
app-color-swatch = { $color } : ressources contenant beaucoup de { $lower }
app-color-any = N’importe quelle couleur
app-color-pick = Choisir une couleur
app-color = Couleur
app-music = Musique
app-tempo = BPM
app-tempo-min = de
app-tempo-max = à
app-tempo-min-title = Musique à ce tempo ou plus rapide, en battements par minute (la moitié ou le double compte aussi)
app-tempo-max-title = Musique à ce tempo ou plus lente, en battements par minute
app-key = Tonalité
app-key-any = N’importe quelle tonalité
app-key-major = Majeur
app-key-minor = Mineur
app-key-mix = Et les tonalités qui se mixent avec elle
app-key-mix-title = Sa relative majeure ou mineure et les tonalités à une quinte de part et d’autre, que les DJ mixent avec elle
app-imitation-placeholder = whooom, pew, krssh
app-imitation-title = Écrivez le bruit que fait un son comme vous l’épelleriez : les sons qui lui ressemblent viennent en premier
app-imitation-record = Faites le bruit dans le micro : les sons qui lui ressemblent viennent en premier
app-imitation-stop = Arrêter l’enregistrement
app-imitation-heard = Votre bruit
app-imitation-mic-failed = Impossible d’enregistrer depuis le micro : { $error }
app-retry = Réessayer
app-undo = Annuler
app-search-failed = La recherche a échoué : { $error }
app-unreachable = Impossible de joindre le serveur : { $error }
app-unreachable-desktop = Hoard ne répond pas : { $error }
app-forget-project-title = Oublier { $name } ?
app-forget-project-body =
    Hoard cesse de l’indiquer là où { $count ->
        [one] sa { $count } ressource est utilisée
       *[other] ses { $count } ressources sont utilisées
    }, jusqu’à ce que son prochain hoard sync le lui signale de nouveau. Rien ne change dans le projet.
app-forget-project-confirm = L’oublier
app-forget-project-failed = Impossible d’oublier { $name } : { $error }
app-forget-project-label = Oublier le projet { $name }
app-forget-project = Oublier ce projet
app-save-name-like = semblable à { $name }
app-save-name-favorites = favoris
app-search-saved = « { $name } » enregistrée : elle se trouve dans Filtres, sous Les vôtres
app-search-updated = « { $name } » mise à jour
app-rank-alike = Les plus semblables
app-rank-color = Le plus de cette couleur
app-rank-match = Meilleure correspondance
app-rank-imitation = Ceux qui y ressemblent le plus
app-sort = Trier
app-sort-name = Nom
app-sort-created = Création
app-sort-updated = Mise à jour
app-sort-size = Taille
app-sort-rating = Note
app-sort-newest-first = Plus récentes d’abord
app-sort-oldest-first = Plus anciennes d’abord
app-sort-a-to-z = De A à Z
app-sort-z-to-a = De Z à A
app-sort-largest-first = Plus grandes d’abord
app-sort-smallest-first = Plus petites d’abord
app-sort-best-rated-first = Mieux notées d’abord
app-sort-lowest-rated-first = Moins bien notées d’abord
app-collection-added =
    { $count ->
        [one] { $count } ressource ajoutée
       *[other] { $count } ressources ajoutées
    } à { $name }
app-collection-add-failed = Impossible d’ajouter à la collection : { $error }
app-collection-removed =
    { $count ->
        [one] { $count } ressource retirée
       *[other] { $count } ressources retirées
    } de { $name }
app-collection-remove-failed = Impossible de retirer de la collection : { $error }
app-ns-subject = Sujet
app-ns-style = Style
app-ns-theme = Thème
app-ns-material = Matériau
app-ns-map = Carte PBR
app-ns-color = Couleur
app-ns-mood = Ambiance
app-ns-genre = Genre
app-ns-license = Licence
app-ns-res = Résolution
app-ns-duration = Durée
app-ns-poly = Nombre de polygones
app-ns-format = Format
app-ns-weight = Poids
app-ns-tech = Technique
app-ns-blender = Version de Blender
app-ns-depth = Profondeur de bits
app-ns-ucs = Catégorie de son (UCS)
app-ns-bpm = Tempo
app-ns-root = Note fondamentale
app-ns-level = Volume sonore
app-ns-ue = Classe Unreal
app-ns-engine = Moteur
app-ns-extracted = Source récupérée
app-ns-script = Système d’écriture
app-bulk-failed =
    Impossible de modifier { $count ->
        [one] cette ressource
       *[other] ces { $count } ressources
    } : { $error }
# Copying the tags of the asset with the keyboard on it (C), to paste onto others (V).
app-tags-copied =
    { $count ->
        [one] { $count } étiquette copiée
       *[other] { $count } étiquettes copiées
    }
app-tags-copy-none = { $name } n'a aucune étiquette à copier
# Pasting (V) with nothing copied, or with copied text that isn't a list of tags.
app-tags-paste-none = Aucune étiquette copiée. Appuyez sur C sur une ressource pour copier ses étiquettes.
# Pasting tags that every one of the assets already has.
app-tags-paste-had =
    { $count ->
        [one] Elle a déjà ces étiquettes
       *[other] Les { $count } ont déjà ces étiquettes
    }
# After pasting: $count tags were added to $assets assets.
app-tags-pasted =
    { $assets ->
        [one]
            { $count ->
                [one] { $count } étiquette ajoutée
               *[other] { $count } étiquettes ajoutées
            }
       *[other]
            { $count ->
                [one] { $count } étiquette ajoutée à { $assets } ressources
               *[other] { $count } étiquettes ajoutées à { $assets } ressources
            }
    }
app-bulk-untagged =
    Étiquette { $tag } retirée de { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }
app-bulk-tagged =
    { $count ->
        [one] { $count } ressource étiquetée
       *[other] { $count } ressources étiquetées
    } avec { $tag }
app-bulk-review = Marquer comme revues
app-bulk-review-title = Marquer les ressources sélectionnées comme passées en revue : les règles et l’IA ne toucheront plus à leurs étiquettes
app-bulk-unreview = Retirer la marque
app-bulk-unreview-title = Retirer la marque « revue » des ressources sélectionnées
app-bulk-reviewed =
    { $count ->
        [one] { $count } ressource marquée comme revue
       *[other] { $count } ressources marquées comme revues
    }
app-bulk-unreviewed =
    { $count ->
        [one] Marque « revue » retirée de { $count } ressource
       *[other] Marque « revue » retirée de { $count } ressources
    }
app-bulk-accept = Garder les suggestions
app-bulk-accept-title = Garder comme étiquettes personnelles toutes celles que les règles et l’IA ont ajoutées aux ressources sélectionnées
app-bulk-accepted =
    { $count ->
        [one] Étiquettes suggérées conservées sur { $count } ressource
       *[other] Étiquettes suggérées conservées sur { $count } ressources
    }
app-credits-failed = Impossible de générer les crédits : { $error }
app-archived-assets =
    { $count ->
        [one] { $count } ressource archivée
       *[other] { $count } ressources archivées
    }
app-archived-one = { $name } archivé
app-open-failed = Impossible d’ouvrir cette ressource : { $error }
app-unnamed-pack = pack
app-unnamed-collection = collection
app-unnamed-asset = ressource
app-home-version = Hoard { $version } — retour à la bibliothèque
app-home = Retour à la bibliothèque
app-search-label = Rechercher des ressources
app-search-placeholder = Rechercher noms, chemins, étiquettes
# The search box's tooltip: what can be typed in it besides words. Keep the examples (rock, stone, moss, "dry moss") in
# the language, and keep the `or`, the minus and the quotes as they are: the search reads those.
app-search-syntax-title = roche or pierre : l’un ou l’autre · -mousse : exclure · "mousse sèche" : ces mots ensemble · pack:forêt : dans le nom d’un pack
app-alt-leave-out = Alt-clic pour l’exclure
app-views = Vues
app-library = Bibliothèque
app-packs = Packs
app-packs-working = { $count } en cours
app-update-title = Hoard { $version } est sorti (ce serveur exécute { $current }). Récupérez la nouvelle image ou le nouveau binaire et redémarrez ; la base de données se met à jour d’elle-même.
app-update = Mise à jour
app-theme = Thème
app-sign-in = Se connecter
app-archive-title = Archives : { $contents }
app-archive = Archives
app-settings = Paramètres
app-palette-title = Palette de commandes ({ $keys })
app-import-folder = Importer un dossier
app-add-files = Ajouter des fichiers
app-add-files-short = Ajouter
app-split-label = Comment importer ce dossier
app-split-before = { "" }
app-split-after = { " " }contient { $count } dossiers ou archives.
app-split-each = Importer chacun comme un pack distinct ({ $count })
app-split-one = Importer comme un seul pack
app-cancel = Annuler
app-filters = Filtres
app-done = Terminé
app-yours = Les vôtres
app-favorites = Favoris
app-needs-review = À revoir
app-needs-review-title = Uniquement les ressources que personne n’a encore passées en revue (se combine avec les autres filtres)
# A filter in the Filters column: only assets with at least as many stars as the one picked beside it.
app-rated = Notées
# Each of the five stars beside Rated, and what picking it keeps.
app-rated-min =
    { $count ->
        [one] { $count } étoile ou plus
       *[other] { $count } étoiles ou plus
    }
# The star picked now, which picked again shows assets of any rating.
app-rated-any = Cliquez à nouveau pour afficher toutes les notes
app-forget-search-label = Oublier la recherche enregistrée { $name }
app-forget-search = Oublier cette recherche
# Beside each saved search in Filters: a house mark that makes it what the library opens to (only one is), and renaming it.
app-default-search-title = Ouvrir la bibliothèque sur cette recherche
app-default-search-on-title = La bibliothèque s’ouvre sur cette recherche. Cliquez pour arrêter.
app-default-search-label = Ouvrir la bibliothèque sur { $name }
app-rename-search = Renommer
app-rename-search-label = Nouveau nom pour { $name }
app-projects-title = Projets qui font hoard sync depuis ce serveur, avec ce que contenait leur dernière synchronisation
app-projects = Projets
app-project-synced = Synchronisé { $when }
app-project-synced-by = Synchronisé { $when } par { $who }
app-collections = Collections
app-no-filters = Aucun filtre pour cette recherche.
app-kind = Type
app-facets-more = { $count } de plus
app-facets-fewer = moins
app-searching = Recherche…
app-searching-live = Recherche en cours
app-asset-count =
    { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }
app-clear-all = Tout effacer
app-save-name-label = Nom de cette recherche
app-save-name-placeholder = Nommez-la
app-save = Enregistrer
app-save-search-title = Garder cette recherche dans Filtres, sous Les vôtres
app-save-search = Enregistrer la recherche
# Beside Save search, after a saved search was chosen and then changed: keep the changes in that saved search instead.
# Save search then reads "Save as new".
app-update-search = Mettre à jour « { $name } »
app-update-search-title = Conserver la sélection actuelle dans cette recherche enregistrée
app-save-search-new = Enregistrer comme nouvelle
app-select-title = Choisir plusieurs ressources pour les étiqueter, catégoriser, mettre en collection, télécharger ou archiver ensemble (X, ou Ctrl-clic)
app-select = Sélectionner
app-tile-size = Taille des vignettes
app-supporting-title = Tampons glTF, bibliothèques de matériaux, cartes des matériaux, matériaux portés par un seul modèle et fichiers de boutique décrivant des modèles. Ils sont inclus quand vous téléchargez ce qui les utilise. Les pages d’une feuille de sprites, et ses sprites que le pack contient aussi en images séparées, sont montrés avec la feuille. Un asset que le pack contient en plusieurs formats est montré une fois, dans le format que Hoard lit le mieux.
app-supporting-hidden =
    { $count ->
        [one] { $count } fichier annexe masqué
       *[other] { $count } fichiers annexes masqués
    }
app-supporting-show = Afficher
app-supporting-shown = Fichiers annexes inclus
app-supporting-hide = Masquer
app-folders = Dossiers
app-folders-title = Parcourir le pack tel qu’il était sur le disque, dossier par dossier
app-folders-label = Dossiers du pack
app-welcome-title = Déposez un pack de ressources n’importe où sur cette page
app-welcome-before = Zips, tarballs, dossiers ou fichiers isolés : tout fonctionne. Hoard les décompresse, stocke chaque fichier une seule fois d’après son hachage, lit les tailles, durées et nombres de polygones, et étiquette tout pour que vous puissiez chercher par exemple{ " " }
app-welcome-example-1 = grass footsteps
app-welcome-example-2 = pbr stone
app-welcome-or = { " " }ou{ " " }
app-welcome-after = .
app-welcome-import-folder = Importer un dossier
app-welcome-choose-files = Choisir des fichiers
app-none-similar = Rien d’autre ne ressemble encore vraiment à { $name }.
app-none = Aucun résultat.
app-none-query = Aucun résultat pour « { $query } ».
app-none-imitation = Aucun son mesuré ne ressemble encore vraiment à « { $word } ».
app-none-heard = Aucun son mesuré ne ressemble encore vraiment à votre bruit.
app-none-filters = Aucun résultat avec ces filtres.
app-none-query-filters = Aucun résultat pour « { $query } » avec ces filtres.
app-clear-search-filters = Effacer la recherche et les filtres
app-loading-more = Chargement de la suite
app-keys-hint = Les flèches déplacent, Entrée ouvre, Espace joue un son, X ou Ctrl-clic sélectionne plusieurs éléments (appui long sur écran tactile).
app-resize-label = Redimensionner le panneau de détails
app-resize-title = Faites glisser pour redimensionner · double-cliquez pour la largeur adaptée
app-selected-label = Ressources sélectionnées
app-selected =
    { $count ->
        [one] { $count } sélectionnée
       *[other] { $count } sélectionnées
    }
# The button on the bar of picked assets that opens the editor for all of them at once.
app-edit-selected = Modifier…
app-edit-selected-title = Modifier les étiquettes et les champs de toutes (E)
# Every match of the search is picked, including those not loaded yet.
app-selected-all =
    { $count ->
        [one] { $count } résultat sélectionné
       *[other] Les { $count } résultats sélectionnés
    }
app-pick-assets = Choisir des ressources
app-download-zip = Télécharger le zip
app-credits-title = CREDITS.md des packs d’où elles viennent : qui les a créées, leurs licences et les mentions qu’ils demandent
app-credits = Crédits
app-prefer-sources-title = SoundWaves et textures Unreal dont Hoard a récupéré le fichier d’origine
app-prefer-sources = WAV/PNG récupérés au lieu de .uasset ({ $count })
bag-clear = Vider la sélection
bag-glb-heading = Modèles en GLB
bag-glb-text = Chaque modèle en fichier .glb, debout avec Y vers le haut et tourné vers l’avant, ses textures en fichiers séparés dans un dossier textures que partagent les modèles. Tout le reste y va tel quel.
bag-glb-download = Télécharger le zip GLB
bag-plain-heading = Fichiers tels quels
bag-plain-text = Chaque fichier tel qu’il a été importé, avec ce dont il a besoin, dans un dossier par pack.
bag-remove = Retirer { $name } de la sélection
bag-more = et { $count } de plus
app-bulk-tag-label = Une étiquette à ajouter aux ressources sélectionnées, ou à leur retirer
app-bulk-tag-placeholder = Étiquette
app-bulk-tag-add-title = Ajouter cette étiquette à chaque ressource sélectionnée
app-bulk-tag-add = Ajouter
app-bulk-tag-remove-title = Retirer cette étiquette de chaque ressource sélectionnée
app-bulk-tag-remove = Retirer
app-remove-from-collection = Retirer de { $name }
app-unfavorite = Retirer des favoris
app-favorite = Ajouter aux favoris
app-select-all = Tout sélectionner ({ $count })
# Picks every match of the search, including those not loaded yet, to edit them all.
app-select-matching =
    { $count ->
        [one] Sélectionner le résultat ({ $count })
       *[other] Sélectionner les { $count } résultats
    }
# The dialog that edits many picked assets at once (E). Its heading.
bulk-edit-title =
    { $count ->
        [one] Modifier { $count } ressource
       *[other] Modifier { $count } ressources
    }
# Its heading when editing every match of a search.
bulk-edit-title-all =
    { $count ->
        [one] Modifier { $count } résultat
       *[other] Modifier les { $count } résultats
    }
bulk-edit-close = Fermer
bulk-edit-tags = Étiquettes
bulk-edit-tag-add = Ajouter une étiquette à toutes
# Beside a tag: how many of the assets being edited have it, e.g. "3 of 12".
bulk-edit-tag-count = { $have } sur { $count }
bulk-edit-untick = Tout décocher
bulk-edit-untick-title = Retirer toutes les étiquettes, puis cocher celles à garder
# Editing every match of a search: the tags listed are only the most used among them.
bulk-edit-tags-most-used = Les étiquettes les plus utilisées parmi elles
# In a field whose value differs between the assets: each keeps its own unless something is typed.
bulk-edit-mixed = Valeurs différentes, laissées telles quelles
# In a field when editing every match of a search: each keeps its own unless something is typed.
bulk-edit-keep = Laissé tel quel
# In a field that was emptied: saving clears it on all of them.
bulk-edit-cleared = Effacé sur toutes
bulk-edit-reset = Rétablir
bulk-edit-all-note = Cela modifie tous les résultats, y compris ceux qui ne sont pas encore chargés, et ne peut pas être annulé en une seule fois.
bulk-edit-save =
    { $count ->
        [one] Enregistrer { $count } modification
       *[other] Enregistrer { $count } modifications
    }
bulk-edit-nothing = Rien n'a changé
bulk-edit-saved =
    { $count ->
        [one] { $count } ressource mise à jour
       *[other] { $count } ressources mises à jour
    }
app-archive-selected-title = Hors des recherches jusqu’à ce qu’elles soient restaurées ou supprimées définitivement, depuis les Archives
app-drop-items = Déposez ces { $count } éléments pour les ajouter
app-drop = Déposez pour ajouter à Hoard
app-drop-hint = Chaque fichier ou dossier de premier niveau devient un pack distinct. Les gigaoctets ne posent pas de problème ; les gros fichiers reprennent si la connexion est coupée.

## Palette de commandes (CommandPalette.vue, avec ses commandes depuis App.vue)

palette-group-go-to = Aller à
palette-group-actions = Actions
palette-group-theme = Thème
palette-group-kinds = Types
palette-group-packs = Packs
palette-group-collections = Collections
palette-group-projects = Projets
palette-group-saved = Recherches enregistrées
palette-group-assets = Ressources
palette-hint-current = actuel
palette-library = Bibliothèque
palette-library-keywords = ressources grille accueil assets grid home
palette-favorites = Favoris
palette-favorites-keywords = favori étoile étoiles aimés starred stars liked
palette-packs = Packs
palette-archive = Archives
palette-archive-keywords = corbeille supprimés restaurer trash recycle bin deleted restore
palette-stats = Statistiques de la bibliothèque
palette-stats-keywords = statistiques chiffres taille espace disque nombre statistics numbers size disk space count
palette-copies = Quasi-copies
palette-copies-keywords = doublons identiques similaires redimensionnés convertis espace perdu duplicates same similar resized converted wasted space dedupe
palette-search = Rechercher
palette-add-files = Ajouter des fichiers…
palette-add-files-keywords = envoyer importer téléverser upload import
palette-import-folder = Importer un dossier…
palette-import-folder-keywords = envoyer pack upload
palette-select-all = Sélectionner les { $count } affichées
palette-select-all-keywords = cocher tout check
palette-select-several = Sélectionner plusieurs…
palette-select-several-keywords = lot plusieurs multiple étiquette archiver bulk tag archive
palette-archive-selected =
    { $count ->
        [one] Archiver la { $count } sélectionnée
       *[other] Archiver les { $count } sélectionnées
    }
palette-archive-selected-keywords = supprimer retirer corbeille delete remove trash
palette-edit-selected = Modifier les { $count } sélectionnées
# Words that find the action above when typed into the command palette.
palette-edit-selected-keywords = modification groupée étiquettes catégorie licence note
palette-paste-tags =
    { $count ->
        [one] Coller les étiquettes copiées sur la ressource sélectionnée
       *[other] Coller les étiquettes copiées sur les { $count } sélectionnées
    }
palette-clear-selection = Effacer la sélection
palette-clear-filters = Effacer les filtres
palette-clear-filters-keywords = réinitialiser reset
palette-hide-supporting = Masquer les fichiers annexes
palette-show-supporting = Afficher les fichiers annexes
palette-supporting-keywords = cartes normales rugosité masques normal maps roughness masks
palette-account = Compte et jetons d’accès
palette-account-keywords = mot de passe connexion utilisateur jeton password login user token api
palette-sign-out = Se déconnecter ({ $name })
palette-sign-out-keywords = déconnexion sortir log out logout
palette-sign-in = Se connecter
palette-sign-in-keywords = connexion compte mot de passe log in login account password
palette-settings = Paramètres
palette-settings-keywords = blender ffmpeg llm ia étiquetage unreal admin administrateur outils préférences options ai tagging tools preferences
palette-shortcuts = Raccourcis clavier
palette-shortcuts-keywords = aide touches clavier help keys
palette-theme-keywords = thème apparence mode clair sombre theme appearance
palette-theme-variant-keywords = apparence couleur appearance color
palette-light-theme = Thème clair : { $name }
palette-dark-theme = Thème sombre : { $name }
palette-kind-all = Tous les types (ne plus afficher seulement les { $kind })
palette-kind-only = Seulement les { $kind }
palette-title = Palette de commandes
palette-placeholder = Rechercher des ressources, aller à un pack, lancer une commande…
palette-results = Résultats
palette-searching = Recherche…
palette-no-match = Aucun résultat pour « { $query } »
palette-foot-move = se déplacer
palette-foot-run = exécuter
palette-foot-shortcuts = tous les raccourcis

## Raccourcis clavier (ShortcutsDialog.vue)

shortcuts-title = Raccourcis clavier
shortcuts-close = Fermer
shortcuts-anywhere = Partout
shortcuts-palette = Palette de commandes : rechercher, naviguer, exécuter
shortcuts-search = Rechercher
shortcuts-this-list = Cette liste
shortcuts-escape = Fermer, effacer la sélection ou fermer les détails
shortcuts-library = Bibliothèque
shortcuts-move = Passer d’une vignette à l’autre (ou J et K)
shortcuts-open = Ouvrir les détails
shortcuts-play-sound = Lire ou arrêter un son
shortcuts-select = Sélectionner ou désélectionner
shortcuts-favorite = Ajouter aux favoris ou en retirer
shortcuts-reviewed = Marquer comme revue ou non
shortcuts-tag = Ajouter une étiquette
shortcuts-edit = Modifier la sélection
shortcuts-copy-tags = Copier ses étiquettes
shortcuts-paste-tags = Coller les étiquettes copiées sur la sélection
shortcuts-key-click = clic
shortcuts-select-range = Sélectionner une plage
shortcuts-select-all = Tout sélectionner parmi ce qui est affiché
shortcuts-sound-player = Lecteur audio
shortcuts-play-pause = Lecture ou pause
shortcuts-skip = Reculer ou avancer
shortcuts-start-end = Début ou fin
shortcuts-image-inspector = Inspecteur d’image
shortcuts-zoom = Zoom avant ou arrière
shortcuts-fit = Ajuster à la fenêtre
shortcuts-actual-pixels = Taille réelle
shortcuts-tile = Répéter en 3×3 pour vérifier les raccords
shortcuts-background = Arrière-plan suivant
shortcuts-channel = Afficher un seul canal

## KindChips.vue

kinds-group = Types de ressources

## TagChip.vue

tag-title-with-source = { $tag } ({ $source })
tag-source-human = ajoutée par une personne
tag-source-rule = de l’étiqueteur hors ligne
tag-source-meta = mesurée à partir du fichier
tag-source-ai = suggérée par l’IA
tag-remove = Retirer l’étiquette { $tag }
tag-accept = Garder l’étiquette { $tag }
tag-accept-title = Garder cette suggestion comme étiquette personnelle
tag-reject-title = La retirer ; l’étiqueteur ne la remettra pas
# In the list offered while typing a tag: before a tag nobody has used yet, which adding will create.
tag-suggest-new = Nouvelle étiquette
# Heading over tags in use that look like the new one typed (a plural, another spelling, a typo).
tag-suggest-near = Vouliez-vous dire
# After a tag offered because what was typed is an old name that was merged into it.
tag-suggest-alias = était { $alias }

## ActivityIndicator.vue

activity-label-idle = Activité en arrière-plan : inactive
activity-label-running = Activité en arrière-plan : { $running } en cours
activity-label-queued = Activité en arrière-plan : { $running } en cours, { $queued } en attente
activity-title = Activité en arrière-plan
activity-unreachable = Impossible de joindre le serveur
activity-unreachable-desktop = Hoard ne répond pas
activity-empty = Rien en cours. Les importations, l’étiquetage, les conversions et les téléchargements de zip apparaissent ici pendant leur exécution.
activity-queued = en attente
activity-stop = Arrêter « { $what } »
activity-stop-tagging = Arrêter tout l’étiquetage par IA ({ $count })
activity-seconds = { $s } s
activity-minutes = { $min } min
activity-hours = { $h } h { $min } min
activity-percent = { $pct } %

## AssetTile.vue

tile-triangles = { $count } tris
tile-label = { $name }, { $category }
tile-label-detail = { $name }, { $category }, { $detail }
tile-play = Lire { $name }
tile-pause = Mettre { $name } en pause
tile-favorite = Ajouter { $name } aux favoris
tile-in-favorites = Dans vos favoris
tile-add-favorite = Ajouter à vos favoris (F)
tile-more-like = Plus de ressources comme { $name }
tile-more-like-this = Plus de ressources similaires
tile-select = Sélectionner { $name }

## CollectionPicker.vue

collections-add = Ajouter à une collection
collections-menu = Collections
collections-find-label = Trouver ou créer une collection
collections-find-placeholder = Trouvez ou nommez une nouvelle collection
collections-create = Créer
collections-empty = Aucune collection pour l’instant. Saisissez un nom pour en commencer une.

## FolderPicker.vue

folders-list = Dossiers
folders-close = Fermer
folders-browse = Parcourir…
folders-up-title = Remonter d’un dossier
folders-up = Remonter
folders-this-computer = Cet ordinateur
folders-none = Aucun dossier ici.

## LocationForm.vue

locations-mode-question = Que doit faire Hoard de ce dossier ?
locations-mode-managed = Le revendiquer : Hoard y garde mes packs
locations-mode-managed-hint = Les packs que vous envoyez ou importez y sont copiés, et Hoard les tient en ordre. Le dossier appartient désormais à Hoard : ne touchez pas à son contenu.
locations-mode-watched = Le surveiller : mes packs y sont déjà
locations-mode-watched-hint = Chaque dossier qu’il contient devient un pack. Hoard lit les fichiers là où ils sont, sans jamais les modifier, déplacer ni supprimer, et prend en compte les nouveaux dossiers dès qu’ils apparaissent.
locations-folder-desktop = Dossier
locations-folder-placeholder = Le dossier où se trouvent vos packs
locations-folder-hint-managed = Hoard le crée s’il n’existe pas. Avec Docker, c’est un dossier que vous avez monté dans le conteneur.
locations-folder-hint-managed-desktop = Hoard le crée s’il n’existe pas.
locations-folder-hint-watched = Avec Docker, montez d’abord le dossier dans le conteneur (en lecture seule, c’est très bien), puis choisissez-le ici.
locations-folder-hint-watched-desktop = Hoard ne fait que le lire : rien n’y est modifié.
locations-name = Nom
locations-name-placeholder-managed = Bibliothèque
locations-name-placeholder-watched = Mes packs
locations-files = Fichiers qui vont ici
locations-catch-all = Ainsi que tout ce qu’aucun autre emplacement ne prend
locations-kinds-hint = Les fichiers d’un nouveau pack vont dans l’emplacement qui coche leur type. Le reste va dans l’emplacement par défaut.
locations-files-all = Tous, pour l’instant. Ajoutez d’autres emplacements plus tard depuis les Paramètres pour envoyer certains types (la musique, par exemple) sur un autre disque.
locations-first-watched-note = L’envoi de fichiers nécessite une bibliothèque gérée par Hoard. Vous pourrez en ajouter une plus tard depuis Paramètres → Emplacements de la bibliothèque.
locations-checking = Vérification du dossier…
locations-add = Ajouter l’emplacement
locations-cancel = Annuler

## LocationsPanel.vue

locations-failed = { $what } : { $error }
locations-load-failed = Impossible de charger les emplacements de la bibliothèque
locations-takes-and-rest = { $kinds }, et tout le reste
locations-takes-rest = Tout ce que les autres ne prennent pas
locations-takes-everything = Tout
locations-takes-nothing = Rien pour l’instant
locations-save-failed = Impossible d’enregistrer l’emplacement
locations-unwatch-title = Arrêter de surveiller { $name } ?
locations-unwatch-body =
    { $count ->
        [one] Son { $count } pack quitte
       *[other] Ses { $count } packs quittent
    } Hoard, avec leurs étiquettes et leurs notes. Les fichiers du dossier ne sont pas touchés.
locations-unwatch-body-empty = Les fichiers du dossier ne sont pas touchés.
locations-unwatch = Arrêter la surveillance
locations-remove-title = Retirer { $name } ?
locations-remove-body = Les fichiers qu’il contient sont d’abord déplacés vers les autres emplacements, selon leur type ; il disparaît une fois vide. Rien n’est perdu.
locations-remove-body-only = S’il contient des fichiers, ajoutez d’abord un autre emplacement vers lequel les déplacer.
locations-remove = Retirer
locations-removed = { $name } retiré
locations-emptying = Déplacement des fichiers hors de { $name } ; il sera retiré une fois terminé
locations-offline-title = { $name } est injoignable
locations-offline-body = Ses fichiers ne peuvent pas en être déplacés tant qu’il est hors ligne. Le retirer quand même ? Les fichiers qui s’y trouvent encore apparaîtront comme manquants.
locations-remove-anyway = Retirer quand même
locations-remove-failed = Impossible de retirer { $name }
locations-rebalancing = Déplacement des fichiers stockés vers les emplacements qui prennent leur type
locations-rebalance-failed = Impossible de déplacer les fichiers
locations-scanned-never = pas encore analysé
locations-scanned-now = analysé à l’instant
locations-scanned-minutes = analysé il y a { $mins } min
locations-scanned-hours = analysé il y a { $hours } h
locations-scanned-date = analysé le { $date }
locations-scan-failed = Impossible d’analyser { $name }
locations-watching = Surveillance de { $name }
locations-added = { $name } ajouté
locations-loading = Chargement…
locations-managed-title = Bibliothèques gérées par Hoard
locations-watched-title = Dossiers surveillés
locations-managed-hint = Les envois et importations sont copiés ici. Chaque emplacement prend les types qu’il coche ; l’emplacement par défaut prend le reste.
locations-watched-hint = Dossiers de packs indexés là où ils sont. Hoard ne modifie jamais leurs fichiers, et cherche les dossiers ajoutés ou supprimés toutes les 15 minutes.
locations-managed-empty = Aucune pour l’instant : un envoi n’a donc nulle part où aller. Ajoutez-en une ci-dessous.
locations-reachable = Joignable
locations-unreachable-now = Injoignable pour le moment
locations-catch-all-tag = par défaut
locations-packs =
    { $count ->
        [one] { $count } pack
       *[other] { $count } packs
    }
locations-unreachable = injoignable
locations-scanning = Analyse…
locations-scan = Analyser maintenant
locations-edit = Modifier
locations-catch-all-locked = Pour changer cela, faites d’un autre emplacement l’emplacement par défaut
locations-edit-hint-before = S’applique aux fichiers ajoutés à partir de maintenant. Pour déplacer ce qui est déjà stocké, utilisez
locations-edit-hint-after = ci-dessous.
locations-save = Enregistrer
locations-rebalance = Déplacer les fichiers stockés en conséquence
locations-rebalance-hint = Après avoir changé ce qu’un emplacement prend : les fichiers déjà stockés vont dans l’emplacement qui prend désormais leur type.
locations-add-title = Ajouter un emplacement
locations-add-hint = Un autre disque pour certains types de fichiers, ou un dossier de packs que vous avez déjà.
locations-kind-question = Quel type d’emplacement est-ce ?
locations-kind-folder = Un dossier
locations-kind-folder-hint = Sur un disque ou un partage NAS monté dans le conteneur, ou un dossier que Dropbox, Google Drive, OneDrive ou iCloud Drive garde synchronisé. Hoard peut y garder des fichiers, ou surveiller des packs que vous avez déjà.
locations-kind-folder-hint-desktop = Sur cet ordinateur, un autre disque ou un partage NAS, ou un dossier que Dropbox, Google Drive, OneDrive ou iCloud Drive garde synchronisé. Hoard peut y garder des fichiers, ou surveiller des packs que vous avez déjà.
locations-kind-s3-hint = Un bucket dans Amazon S3 ou un service semblable : Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage. Hoard y garde des fichiers.
locations-s3-title = Stockage S3
locations-s3-hint = Un emplacement s3://bucket-name est un bucket de ce service : AWS, ou tout service compatible S3 (Backblaze B2, Cloudflare R2, Wasabi, MinIO, Garage sur un NAS).
locations-bucket = Bucket
locations-bucket-hint = Le nom d’un bucket qui existe déjà. Les clés ci-dessus doivent pouvoir le lister, le lire et y écrire.
locations-s3-endpoint = Adresse
locations-s3-endpoint-placeholder = Vide pour AWS ; p. ex. https://s3.eu-central-003.backblazeb2.com
locations-s3-region = Région
locations-s3-access-key = ID de clé d’accès
locations-s3-secret-key = Clé d’accès secrète
locations-s3-headers = En-têtes supplémentaires
locations-s3-headers-placeholder = Name=value, un par ligne (pour Cloudflare Access, par exemple)
locations-s3-kept = Enregistré (tapez pour remplacer)
locations-s3-secrets = Les clés et les en-têtes sont conservés avec les paramètres du serveur et ne sont plus jamais affichés.
locations-s3-secrets-desktop = Les clés et les en-têtes sont conservés dans le trousseau de cet ordinateur et ne sont plus jamais affichés.
locations-s3-saved = Stockage S3 enregistré
locations-s3-save-failed = Impossible d’enregistrer le stockage S3
locations-add-more = Ajouter un emplacement…

## IssuesPanel.vue

issues-failed = { $what } : { $error }
issues-load-failed = Impossible de charger les Problèmes
issues-started =
    { $count ->
        [one] Relancé.
       *[other] { $count } éléments relancés.
    }
issues-retry-failed = Impossible de réessayer
issues-dismiss-failed = Impossible d’ignorer
issues-dismiss-all-title = Ignorer les { $count } ?
issues-dismiss-all-body = « { $title } » est vidé. Tout ce qui échoue de nouveau y sera de nouveau listé.
issues-dismiss-all = Tout ignorer
issues-just-now = à l’instant
issues-minutes-ago = il y a { $mins } min
issues-hours-ago = il y a { $hours } h
issues-loading = Chargement…
issues-none = Aucun problème.
issues-action-all = { $action } (tous)
issues-open-asset = L’ouvrir
issues-open-pack = Afficher le pack
issues-open-locations = Emplacements de la bibliothèque
issues-where-before = dans
issues-where-after = { "" }
issues-times =
    { $count ->
        [one] { $count } fois
       *[other] { $count } fois
    }
issues-starting = Démarrage…
issues-dismiss-named = Ignorer { $title }
issues-dismiss = Ignorer
issues-show-fewer = Afficher moins
issues-show-all = Tout afficher ({ $count })
issues-not-listed = { $count } autres ne sont pas listés.

## archive.ts

archive-anyway = Archiver quand même
archive-failed = Impossible d’archiver : { $error }
archive-undo = Annuler
archive-restore-failed = Impossible de restaurer : { $error }
archive-count-packs-and-assets =
    { $packs ->
        [one] { $packs } pack
       *[other] { $packs } packs
    } et { $assets ->
        [one] { $assets } ressource
       *[other] { $assets } ressources
    }
archive-count-packs =
    { $packs ->
        [one] { $packs } pack
       *[other] { $packs } packs
    }
archive-count-assets =
    { $assets ->
        [one] { $assets } ressource
       *[other] { $assets } ressources
    }
archive-count-nothing = rien

## ArchiveView.vue

archive-load-failed = Impossible de charger les archives : { $error }
archive-warning-with-uses = { $uses } { $body }
archive-restored = Restauré : { $what }
archive-purged = Supprimé définitivement : { $what }
archive-purge-failed = Impossible de supprimer : { $error }
archive-purge-title = Supprimer définitivement { $name } ?
archive-purge-pack-body =
    { $count ->
        [one] Sa { $count } ressource
       *[other] Ses { $count } ressources
    } et { $size } de fichiers disparaissent aussi, sauf les fichiers partagés avec un autre pack. Cette action est irréversible.
archive-purge = Supprimer définitivement
archive-purge-asset-body = Son fichier est libéré, sauf si une autre ressource le partage. Cette action est irréversible.
archive-empty-title = Vider les archives ?
archive-empty-body = Le contenu des archives ({ $what }) sera supprimé définitivement et ses fichiers libérés. Cette action est irréversible.
archive-empty = Vider les archives
archive-title = Archives
archive-intro = Les packs et ressources archivés sont exclus des recherches, des décomptes et de la liste des étiquettes. Restaurez-les, ou supprimez-les définitivement pour libérer leur espace.
archive-loading = Chargement…
archive-is-empty = Les archives sont vides
archive-is-empty-hint = Archivez un pack ou une ressource : il attend ici jusqu’à ce que vous le restauriez ou le supprimiez définitivement.
archive-packs = Packs
archive-assets = Ressources
archive-asset-count =
    { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }
archive-archived = archivé { $when }
archive-from = de { $pack }
archive-restore = Restaurer
archive-show-more = Afficher plus ({ $count } restants)

## mine.ts

projects-favorite-add-failed = Impossible d’ajouter à vos favoris : { $error }
projects-favorite-remove-failed = Impossible de retirer de vos favoris : { $error }
projects-save-search-failed = Impossible d’enregistrer la recherche : { $error }
projects-forget-search-failed = Impossible de supprimer { $name } : { $error }
projects-change-search-failed = Impossible de modifier { $name } : { $error }

## projects.ts

projects-list-and = { $items } et { $last }
projects-list-separator = ,{ " " }
projects-use = { $name } ({ $files })
projects-use-more = { $count } de plus
projects-other =
    { $count ->
        [one] { $count } autre projet
       *[other] { $count } autres projets
    }
projects-usage-title-one = { $name } utilise ceci
projects-usage-title = { $count } projets utilisent ceci
projects-usage-body-one = Utilisé dans { $list } : son prochain hoard sync les retire.
projects-usage-body = Utilisé dans { $list } : le prochain hoard sync de chacun les retire.

## uploader.ts

upload-network-error = erreur réseau
upload-gave-up = { $what } : { $error } après { $attempts } tentatives
upload-failed-plain = échec
upload-already-imported = déjà importé
upload-part-failed = échec de l’envoi ({ $status })
upload-batch = lot de { $count } fichiers
upload-batch-failed = échec du lot ({ $status })
upload-finish = fin de l’envoi

## UploadQueue.vue

upload-eta-seconds = encore { $s } s
upload-eta-minutes = encore { $min } min
upload-eta-hours = encore { $h } h
upload-queue = Importations
upload-importing =
    { $count ->
        [one] Importation de { $count } pack
       *[other] Importation de { $count } packs
    }
upload-bytes-of = { $sent } sur { $total }
upload-rate = { $size }/s
upload-waiting = En attente
upload-unpacking = Envoyé, décompression en cours
upload-already = Déjà dans la bibliothèque
upload-stopped = Arrêté
upload-files-of = { $done } fichiers sur { $total }
upload-resumed = repris, { $size } déjà présents
upload-skipped = Ces fichiers identiques avaient déjà été importés sous le nom « { $name } », ils ont donc été ignorés.
upload-error = { $error }. Votre progression est conservée : réessayez, ou déposez à nouveau les mêmes fichiers plus tard.
upload-retry = Réessayer
# $name is the pack the library has already that these files go into.
upload-update = Mettre à jour « { $name } »
upload-copy = Importer comme copie
# $name is the pack the library has already that the upload goes into.
upload-into = Va dans « { $name } », déjà dans la bibliothèque : les nouveaux fichiers sont ajoutés et ceux qui ont changé sont remplacés.
upload-own-pack = En faire un pack à part
upload-dismiss = Ignorer
upload-cancel = Annuler
upload-minimize = Réduire
upload-expand = Afficher les importations
upload-pill = Importation de { $count } · { $percent } %
upload-pill-stopped = Arrêtées : { $count }
upload-interrupted = Interrompue
upload-resume = Reprendre
upload-resume-all = Tout reprendre
upload-resume-hint-folder = Choisissez à nouveau le même dossier, ou déposez-le sur la page, pour reprendre là où il s’est arrêté.
upload-resume-hint-file = Choisissez à nouveau le même fichier, ou déposez-le sur la page, pour reprendre là où il s’est arrêté.
upload-not-same = Ce ne sont pas les mêmes fichiers, donc rien n’a été repris.
upload-permission-denied = Le navigateur n’a pas été autorisé à relire les fichiers. Choisissez-les plutôt.

## PacksView.vue

packs-rename-failed = Impossible de renommer : { $error }
packs-collection-deleted = Collection { $name } supprimée
packs-delete-failed = Impossible de supprimer : { $error }
packs-status-uploading = Envoi en cours (fichiers reçus jusqu’ici)
packs-status-ingesting = Décompression et étiquetage
packs-status-analyzing = Étiquetage par le LLM
packs-status-ready = Prêt
packs-status-failed = Échec
packs-retag-failed = Impossible de lancer le réétiquetage : { $error }
packs-read-files-failed = Impossible de lire les fichiers du pack : { $error }
packs-context-load-failed = Impossible de charger les informations de boutique du pack : { $error }
packs-undo-store-failed = Impossible de retirer la page de boutique : { $error }
packs-context-save-failed = Impossible d’enregistrer les informations de boutique : { $error }
packs-unreal-load-failed = Impossible de charger l’état de la conversion Unreal : { $error }
packs-unreal-discard-title = Supprimer les originaux ?
packs-unreal-discard-body = Les fichiers .uasset d’origine de { $name } sont supprimés une fois convertis. Les données propres à Unreal qu’ils contiennent, comme les graphes de matériaux, sont définitivement perdues.
packs-unreal-discard-confirm = Convertir et supprimer
packs-unreal-convert-failed = Impossible de lancer la conversion : { $error }
packs-unreal-fact-converted = { $done } convertis depuis Unreal
packs-unreal-fact-packages =
    { $left ->
        [one] { $left } paquet Unreal
       *[other] { $left } paquets Unreal
    }
packs-unreal-fact-both = { $done } convertis depuis Unreal, { $left } encore en paquets
packs-thumbs-canceled = Annulé après { $done } sur { $total }
packs-thumbs-drew =
    { $count ->
        [one] { $count } modèle dessiné
       *[other] { $count } modèles dessinés
    }
packs-thumbs-result =
    { $improved ->
        [one] { $lead } : { $improved } miniature remplacée.
       *[other] { $lead } : { $improved } miniatures remplacées.
    }
packs-thumbs-result-failed =
    { $improved ->
        [one] { $lead } : { $improved } miniature remplacée, { $failed } n’ont pas pu être dessinées.
       *[other] { $lead } : { $improved } miniatures remplacées, { $failed } n’ont pas pu être dessinées.
    }
packs-archived =
    { $name } archivé, avec { $count ->
        [one] sa { $count } ressource
       *[other] ses { $count } ressources
    }
packs-blobs-bad = { $count } fichiers ont échoué à leur dernière vérification d’intégrité.
packs-collections = Collections
packs-browse-named = Parcourir { $name }
packs-collection-name = Nom de la collection
packs-save = Enregistrer
packs-cancel = Annuler
packs-assets =
    { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }
packs-updated = Mis à jour le { $date }
packs-browse = Parcourir
packs-collection-download = Télécharger (.zip)
packs-collection-credits = Crédits
packs-collection-credits-title = Qui a créé les packs d’où viennent ces ressources, leurs licences et les mentions qu’ils demandent
packs-rename = Renommer
packs-delete = Supprimer
packs-packs = Packs
packs-empty = Aucun pack pour l’instant. Déposez un zip ou un dossier n’importe où sur cette page pour en ajouter un.
packs-find = Trouver un pack
packs-find-placeholder = Trouvez un pack par nom, boutique, licence ou étiquette
packs-sort = Ordre
packs-sort-new = Plus récents d’abord
packs-sort-old = Plus anciens d’abord
packs-sort-size = Plus gros d’abord
packs-sort-assets = Plus de ressources d’abord
packs-sort-count = Nombre de ressources
packs-sort-smallest = Plus petits d’abord
packs-sort-fewest = Moins de ressources d’abord
packs-with-kind = Packs contenant
packs-state = État
packs-state-any = Tous les états
packs-state-working = En cours de traitement
packs-state-failed = En échec
packs-state-untagged = Pas encore décrits
packs-state-unlicensed = Sans licence indiquée
packs-store = Boutique
packs-store-any = Toutes les boutiques
packs-store-from = De { $store }
packs-store-none = Sans page de boutique
packs-shown-of = { $shown } sur { $total }
packs-clear = Effacer
packs-no-match = Aucun pack ne correspond.
packs-clear-all = Effacer la recherche et les filtres
packs-pack-name = Nom du pack
packs-thumbs-drawing = Dessin des miniatures dans ce navigateur…
packs-thumbs-drawing-count = Dessin des miniatures dans ce navigateur : { $done } sur { $total }…
packs-thumbs-stopped = Arrêté : { $error }
packs-thumbs-none = Chaque modèle ici a déjà une miniature dessinée avec toutes ses textures.
packs-dismiss = Ignorer
packs-license = Licence : { $license }
packs-license-tier = Licence : { $license } ({ $tier })
packs-license-not-found = introuvable
packs-from-before = Source :
packs-from-after = { "" }
packs-added = Ajouté le { $date }
pack-panel-label = À propos de ce pack
pack-panel-hide = Masquer les détails du pack
pack-panel-kinds = Ce qu’il contient
pack-panel-all = Tous les packs
packs-download = Télécharger le pack (.zip)
packs-store-details = Informations de boutique
packs-more-named = Plus d’actions pour { $name }
packs-more = Plus
packs-rename-more = Renommer…
packs-retag-rules = Relancer l’étiqueteur hors ligne
packs-retag-llm = Relancer l’étiqueteur LLM
packs-make-thumbs = Créer les miniatures dans ce navigateur
packs-make-thumbs-title = Ouvrir chaque modèle dans une visionneuse 3D invisible, un par un, et utiliser son image comme miniature quand elle est meilleure que celle du serveur
packs-make-thumbs-title-desktop = Ouvrir chaque modèle dans une visionneuse 3D invisible, un par un, et utiliser son image comme miniature quand elle est meilleure que celle créée par Hoard
packs-read-files = Relire les fichiers du pack
packs-credits = Télécharger les crédits
packs-credits-title = CREDITS.md : qui a créé le pack, sa licence et la mention qu’il demande
packs-unreal-open = Convertir le contenu Unreal…
packs-archive = Archiver le pack
packs-unreal-intro = Hoard lit lui-même les paquets Unreal et stocke leur contenu sous forme de fichiers ordinaires : les textures en PNG ou EXR, les sons en WAV, les matériaux en JSON. Le fichier converti prend la place du paquet et en conserve les étiquettes, les liens et les collections.
packs-unreal-converted =
    { $count ->
        [one] converti
       *[other] convertis
    }
packs-unreal-converted-kept =
    { $count ->
        [one] converti, { $kept } avec l’original conservé
       *[other] convertis, { $kept } avec l’original conservé
    }
packs-unreal-pending =
    { $count ->
        [one] pas encore examiné
       *[other] pas encore examinés
    }
packs-unreal-needs-editor =
    { $count ->
        [one] en attente d’Unreal Editor
       *[other] en attente d’Unreal Editor
    }
packs-unreal-unsupported =
    { $count ->
        [one] sans format courant
       *[other] sans format courant
    }
packs-unreal-failed =
    { $count ->
        [one] en échec
       *[other] en échec
    }
packs-unreal-reasons = Pourquoi des paquets restent
packs-unreal-editor-before = Les maillages et les animations nécessitent Unreal Editor. Sur une machine où Unreal est installé, exécutez
packs-unreal-editor-command = hoard unreal-worker --editor <chemin vers UnrealEditor-Cmd>
packs-unreal-editor-after = { " " }; elle les exporte en FBX et envoie les résultats ici.
packs-unreal-discard = Supprimer les fichiers .uasset d’origine une fois convertis
packs-unreal-discard-hint = Libère de l’espace. Les données propres à Unreal, comme les graphes de matériaux, sont perdues.
packs-unreal-drop = Supprimer les paquets sans format courant
packs-unreal-drop-hint = Blueprints, niveaux et autres ressources propres au moteur.
packs-unreal-convert = Convertir
packs-context-hint-before = D’où vient le pack et ce qu’en dit le vendeur. L’étiqueteur LLM lit ceci avec chaque fichier : un texte de boutique comme « 120 sorts de glace et de feu » l’aide à nommer
packs-context-hint-after = .
packs-store-auto = Trouvée automatiquement : le pack mentionne lui-même cette page.
packs-store-not-this = Pas celle-ci
packs-lookup-label = Chercher dans les boutiques
packs-lookup-placeholder = Le nom du pack dans la boutique
packs-lookup-searching = Recherche…
packs-lookup-search = Chercher dans les boutiques
packs-lookup-hide = Masquer
packs-lookup-clues = D’après les fichiers du pack : { $clues }.
packs-lookup-also = Également recherché : { $queries }.
packs-lookup-quoted = « { $query } »
packs-lookup-nothing = Rien sur Fab, itch.io, Kenney ou OpenGameArt ne lui ressemble. Essayez moins de mots, ou d’autres mots.
packs-lookup-use = Utiliser
# Clicking Use on a store match lays it out field by field: what Store details has now beside what the store says,
# each with a checkbox to take it or leave it.
packs-review-now = Actuel
packs-review-store = Depuis la boutique
packs-review-same = Identique
packs-review-read-page = Lu depuis la page à l’enregistrement
packs-review-take =
    { $count ->
        [one] Remplir { $count } champ
       *[other] Remplir { $count } champs
    }
packs-lookup-searched = Dernière recherche : { $date }.
packs-lookup-again = Chercher à nouveau
packs-lookup-again-after = .
packs-lookup-unreachable = Impossible de joindre { $stores } cette fois-ci.
packs-lookup-also-look = Cherchez aussi sur
packs-lookup-links-and = { " " }et{ " " }
packs-lookup-browser-only = (on ne peut y chercher que dans votre navigateur) ; collez l’adresse de la page ci-dessous.
packs-lookup-open = Trouver dans les boutiques…
packs-store-placeholder = Fab, itch.io, Humble Bundle…
packs-seller = Vendeur
packs-seller-placeholder = Qui l’a créé
packs-license-label = Licence
packs-license-placeholder = Fab Standard License, CC BY 4.0…
packs-license-tier-label = Niveau de licence
packs-license-tier-placeholder = Personnel, Professionnel…
# A checkbox under the license: on Save, the pack's assets with a license of their own go by the pack's instead.
packs-own-licenses =
    { $count ->
        [one] { $count } ressource a sa propre licence : lui appliquer aussi celle du pack
       *[other] { $count } ressources ont leur propre licence : leur appliquer aussi celle du pack
    }
packs-store-page = Page de boutique
packs-store-text = Description de la boutique
packs-store-text-placeholder = Collez le texte de l’annonce, ou laissez vide pour le récupérer depuis la page de boutique.
packs-store-text-fetch = L’enregistrement récupère la description depuis la page de boutique.
packs-readme = README trouvé dans le pack
packs-save-retag = Enregistrer et relancer l’étiqueteur LLM

## AssetDetail.vue

asset-lazy-3d-viewer = la visionneuse 3D
asset-lazy-text = le texte
asset-lazy-text-viewer = la visionneuse de texte
asset-lazy-image-viewer = la visionneuse d’images
asset-lazy-material-preview = l’aperçu du matériau
asset-lazy-material-viewer = la visionneuse de matériaux
asset-lazy-sprite-player = le lecteur de sprites
asset-lazy-sheet-view = la visionneuse de planches de sprites
asset-rel-texture = texture
asset-rel-buffer = données de géométrie
asset-rel-material = fichier de matériau
asset-rel-animation = animation
asset-rel-library = fichier lié
asset-rel-guess = va probablement avec
asset-rel-sidecar = informations de boutique
asset-rel-variant = variante
asset-rel-sprite = sprite
asset-rel-sample = échantillon
asset-role-map = carte { $role }
asset-maps-count =
    { $count ->
        [one] { $count } carte
       *[other] { $count } cartes
    }
asset-needs = Nécessite
asset-animations = Animations
asset-sprites = Sprites
asset-used-by = Utilisé par
asset-loose-sprites = Aussi en fichiers séparés
asset-in-sheet = Dans une feuille de sprites
asset-other-formats = Autres formats
asset-shown-as = Montré dans la bibliothèque en
asset-conversion-failed = échec de la conversion
asset-as-converting = Conversion…
asset-as-converting-percent = Conversion… { $percent } %
asset-as-queued = En attente…
asset-download = Télécharger
asset-as-label = Télécharger en
asset-as-original = { $format } (format d’origine)
asset-as-pack-files = Les fichiers du pack
asset-as-size = Taille
asset-as-full-size = Taille réelle
asset-as-up-to = Jusqu’à { $size } px
asset-as-hint = Choisissez une taille ou un autre format
asset-meta-size = Taille
asset-meta-length = Durée
asset-meta-sample-rate = Fréquence d’échantillonnage
asset-meta-channels = Canaux
asset-meta-mono = Mono
asset-meta-stereo = Stéréo
asset-meta-bit-depth = Profondeur de bits
asset-meta-bits = { $bits } bits
asset-meta-level = Niveau
asset-meta-level-value = crête { $peak } dBFS, RMS { $rms } dBFS
asset-meta-level-clips = crête { $peak } dBFS, RMS { $rms } dBFS, saturé
asset-meta-loop = Boucle
asset-meta-loop-value = de { $start } à { $end }
asset-meta-playback = Lecture
asset-meta-one-shot = Lecture unique
asset-meta-envelope = Enveloppe
asset-meta-envelope-times = attaque { $attack }, queue { $tail }
asset-meta-envelope-hit = impact
asset-meta-envelope-swell = crescendo
asset-meta-envelope-sustained = tenu
asset-meta-tone = Timbre
asset-meta-tone-value = brillance { $khz } kHz, bruit { $noise } %
asset-meta-pitch = Hauteur
asset-meta-pitch-rises =
    { $octaves ->
        [one] monte de { $octaves } octave
       *[other] monte de { $octaves } octaves
    }
asset-meta-pitch-falls =
    { $octaves ->
        [one] descend de { $octaves } octave
       *[other] descend de { $octaves } octaves
    }
asset-meta-loops-cleanly = Boucle sans raccord
asset-meta-tempo = Tempo
asset-meta-tempo-heard = environ { $bpm } BPM, à l’écoute
asset-meta-tempo-heard-or = environ { $bpm } BPM (ou { $alt }), à l’écoute
asset-meta-key = Tonalité
asset-meta-key-value = { $key } ({ $camelot })
asset-meta-key-heard = { $key }, à l’écoute
music-key-major = { $note } majeur
music-key-minor = { $note } mineur
asset-meta-root-note = Note fondamentale
asset-meta-cue-markers = Marqueurs
asset-meta-library = Bibliothèque
asset-meta-keywords = Mots-clés
asset-meta-title = Titre
asset-meta-artist = Artiste
asset-meta-album = Album
asset-meta-genre = Genre
asset-meta-generator = Générateur
asset-meta-everything-with = Afficher tout ce qui porte l’étiquette { $tag }
asset-meta-saved-with = Enregistré avec
asset-meta-color-mode = Mode colorimétrique
asset-meta-layers = Calques
asset-meta-range = Plage
asset-meta-hdr = HDR (l’aperçu est tone-mappé)
asset-meta-format = Format
asset-meta-large-document = Grand document (PSB)
asset-meta-ai = IA
asset-meta-ai-made = Créé avec une IA générative
asset-meta-ai-edited = Modifié avec une IA générative
asset-meta-ai-how-metadata = métadonnées
asset-meta-ai-how-generator = paramètres du générateur
asset-meta-ai-how-iptc = type de source numérique IPTC
asset-meta-ai-says = { $made } (source : { $how })
asset-meta-ai-by-says = { $made } ({ $by }) (source : { $how })
asset-meta-provenance = Provenance
asset-meta-c2pa = Contient des Content Credentials C2PA
asset-meta-compression = Compression
asset-meta-note = Note
asset-meta-maps = Cartes
asset-meta-sizes =
    { $count ->
        [one] Taille
       *[other] Tailles
    }
asset-meta-triangles = Triangles
asset-meta-vertices = Sommets
asset-meta-meshes = Maillages
asset-meta-materials = Matériaux
asset-meta-animations = Animations
asset-meta-skins = Skins
asset-meta-unreal-asset = Ressource Unreal
asset-meta-unreal-saved-with = { $class }, enregistré avec UE { $engine }
asset-meta-package = Paquet
asset-meta-cooked = Cuit (build de jeu) : aperçus et sources supprimés
asset-meta-tiling = Répétition
asset-meta-seamless = Sans raccord sur les deux axes
asset-meta-analysis-error = Impossible de lire entièrement ce fichier : { $error }
asset-copy-blocked = Copie impossible : le navigateur l’a bloquée
asset-save-error = Impossible d’enregistrer : { $error }
asset-tag-removed = Étiquette { $tag } retirée
asset-tag-restored = Étiquette { $tag } remise
asset-tags-added =
    { $count ->
        [one] { $count } étiquette ajoutée
       *[other] { $count } étiquettes ajoutées
    }
asset-tags-accepted =
    { $count ->
        [one] { $count } étiquette suggérée conservée
       *[other] { $count } étiquettes suggérées conservées
    }
asset-reviewed = Revue
asset-reviewed-on-title = Revue : les règles et l’IA ne touchent pas à ses étiquettes. Cliquez pour retirer la marque (R)
asset-reviewed-off-title = Marquer comme revue, pour que les règles et l’IA ne touchent pas à ses étiquettes (R)
asset-reviewed-on = Marquée comme revue
asset-reviewed-off = Marque « revue » retirée
# $name is an asset's name.
asset-reviewed-on-named = { $name } marquée comme revue
asset-reviewed-off-named = Marque « revue » retirée de { $name }
asset-undo = Annuler
asset-play-error = Impossible de lire ce fichier : { $error }
asset-copied-hash = Hachage copié
asset-copied-toml = toml copié
asset-copied-id = ID copié
asset-copied-link = Lien copié
asset-origin-before = Ce fichier était
asset-origin-class-engine = , un { $class } enregistré par UE { $engine }.
asset-origin-class = , un { $class }.
asset-origin-engine = { " " }enregistré par UE { $engine }.
asset-origin-end = { "." }
asset-origin-editor = Unreal Editor l’a exporté le { $date }.
asset-origin-editor-notes = Unreal Editor l’a exporté le { $date } ({ $notes }).
asset-origin-hoard = Hoard l’a extrait du paquet le { $date }.
asset-origin-hoard-notes = Hoard l’a extrait du paquet le { $date } ({ $notes }).
asset-origin-download = Télécharger le .{ $ext } d’origine
asset-origin-discarded = Le paquet d’origine a été supprimé après la conversion.
# An asset's history, a section near the bottom of its details: each change made to it, the latest first.
history-heading = Historique
history-loading = Chargement…
history-none = Rien n’a encore été modifié ici.
history-load-failed = Impossible de charger l’historique : { $error }
# Who made a group of changes. $who is a person's name.
history-by-ai = IA
history-by-rule = Une règle
history-by-someone = Quelqu’un
history-by-app = { $who } via une application
history-by-assistant = { $who } via un assistant
history-field-description = Description
# One change to a tag. "Kept" is a person accepting a tag the AI or a rule suggested.
history-tag-added = Étiquette { $tag } ajoutée
history-tag-removed = Étiquette { $tag } retirée
history-tag-kept = Étiquette suggérée { $tag } conservée
history-tag-changed = Étiquette { $tag } modifiée
history-reviewed = Marqué comme vérifié
history-unreviewed = Marqué comme non vérifié
# One change to a field. $field is its name (Name, Category, Rating…), $from and $to what it was and became.
history-set = { $field } : défini sur { $to }
history-cleared = { $field } : effacé (était { $from })
history-changed = { $field } : { $from } → { $to }
# Beside each change, for those who can edit: puts back what it was before that change.
history-revert = Annuler
history-revert-title = Rétablir ce que cette modification a changé
history-reverted = Annulé : { $change }
history-revert-failed = Impossible d’annuler : { $error }
asset-details = Détails de la ressource
asset-in-pack = dans
asset-previous = Ressource précédente
asset-next = Ressource suivante
asset-favorite = Favori
asset-favorite-on-title = Dans vos favoris (F pour la retirer)
asset-favorite-off-title = Ajouter à vos favoris (F)
asset-share = Partager
asset-share-title = Partager un lien vers cette ressource (la personne qui l’ouvre se connecte d’abord à Hoard)
asset-layout-panel = Afficher dans un panneau latéral
asset-layout-page = Afficher en pleine page
asset-layout-page-title = Afficher en pleine page : un grand aperçu à côté des détails
asset-close = Fermer les détails
asset-rename-title = Cliquez pour renommer
asset-name = Nom
asset-views-alt = Le modèle vu de quatre côtés
asset-inspect-aria = Inspecter { $name } en plein écran
asset-inspect-title = Inspecter : zoom, répétition, canaux
asset-playback-position = Position de lecture
asset-playback-value = { $current } sur { $duration }
asset-pause = Pause
asset-play = Lire
asset-waveform-drawing = Dessin de la forme d’onde…
asset-waveform-skipped = Pas de forme d’onde pour des fichiers aussi gros
asset-play-animation = Lire comme une animation
asset-show-views = Afficher la planche à quatre vues qu’a vue l’étiqueteur
asset-palette = Couleurs de l’image
asset-swatch-title = { $hex }, { $percent } % de l’image. Trouver des ressources de cette couleur
asset-swatch-aria = Trouver des ressources avec { $hex }, { $percent } % de cette image
asset-more-like-this = Plus de ressources similaires
asset-desc-label = Votre description
asset-desc-placeholder = Ce que c’est, où vous l’utiliseriez…
asset-save = Enregistrer
asset-cancel = Annuler
asset-desc-family = S’enregistre sur les { $count } variantes
asset-desc-edit = Modifier la description
asset-desc-write = Écrire une description
asset-desc-ai = Écrite par l’IA
asset-desc-file = Tirée du fichier
asset-desc-store-before = D’après la
asset-desc-store-page = page { $store }
asset-desc-store-page-unnamed = page de boutique
asset-desc-store-after = (tout le pack)
asset-less = Moins
asset-more = Plus
asset-tags = Étiquettes
asset-tag-add-label = Ajouter une étiquette
asset-tag-add-placeholder = Ajoutez une étiquette, par ex. mood:tense
asset-apply-family = Appliquer les modifications aux { $count } variantes
asset-tags-accept-all =
    { $count ->
        [one] Garder { $count } suggestion
       *[other] Garder les { $count } suggestions
    }
asset-tags-accept-all-title = Garder comme étiquettes personnelles toutes celles suggérées par les règles et l’IA
# Before a list of tags someone removed, which the taggers won't put back; each can be clicked to restore it.
asset-tags-rejected = Refusées :
asset-tag-restore-title = Remettre { $tag }
# Under the tag box: tags that assets which look like this one have and it doesn't, each added with a click.
asset-tags-from-similar = Les similaires ont :
asset-tag-suggest-title = Ajouter { $tag }
asset-details-heading = Détails
asset-field-category = Catégorie
asset-field-license = Licence
asset-field-source = Source
asset-field-rating = Note
# Shown in place of an empty value, to fill it in.
asset-field-add = Ajouter
asset-field-new = Ajouter un champ
asset-field-name = Nom du champ
asset-field-value = Valeur
# After a license the asset gets from its pack rather than its own.
asset-field-from-pack = du pack
asset-field-license-placeholder = Par exemple CC0 ou CC-BY 4.0
asset-field-remove-placeholder = Vide pour le retirer
# $field is a field's name, like "License".
asset-field-edit-title = Modifier { $field }
asset-field-saved = { $field } enregistré
asset-field-cleared = { $field } effacé
asset-rate =
    { $count ->
        [one] { $count } étoile
       *[other] { $count } étoiles
    }
asset-rated =
    { $count ->
        [one] Noté { $count } étoile
       *[other] Noté { $count } étoiles
    }
asset-rating-clear-title = Cliquez à nouveau pour effacer la note
asset-rating-cleared = Note effacée
# An asset's rating: the star it has now, clicked again, takes it down to one fewer.
asset-rating-down-title =
    { $count ->
        [one] Cliquez à nouveau pour { $count } étoile
       *[other] Cliquez à nouveau pour { $count } étoiles
    }
asset-collections = Collections
asset-collection-remove = Retirer de { $name }
asset-collection-add-another = Ajouter à une autre
asset-collection-add = Ajouter à une collection
asset-used-in = Utilisé dans
asset-used-in-title = Afficher ce que { $name } contient
asset-used-in-synced = synchronisé { $when }
asset-maps = Cartes
asset-show-more = Afficher { $count } de plus
asset-materials-heading =
    { $count ->
        [one] Matériau
       *[other] Matériaux
    }
asset-material-maps-aria = { $name } : cartes
asset-variants = Variantes ({ $count } prises)
asset-fit-animations = Animations compatibles ({ $count })
asset-fit-characters = Personnages compatibles ({ $count })
asset-copies = Copies identiques
asset-copies-note = Les mêmes octets ne sont stockés qu’une fois ; ces packs contiennent aussi ce fichier.
asset-near-copies = Quasi-copies
asset-near-copies-note = La même chose dans une autre taille, fréquence d’échantillonnage ou un autre format. La meilleure copie est signalée.
asset-near-copies-best = La même chose dans une autre taille, fréquence d’échantillonnage ou un autre format. Celle-ci est la meilleure copie.
asset-near-best = meilleure
asset-unreal-converted = Converti depuis Unreal
asset-rule-converted = Converti par une règle
# Followed by the original file's name: "A conversion rule made this GLB on 3 May 2026 from tree.obj".
asset-rule-was = Une règle de conversion a créé ce { $format } le { $date } à partir de
asset-rule-kept-until = L’original est conservé jusqu’au { $date }.
asset-rule-rebuilt = Le FLAC conserve tout ce qui se trouvait dans le WAV : le WAV peut donc être téléchargé ou remis exactement tel qu’il était à tout moment.
asset-rule-put-back = Remettre l’original
asset-rule-put-back-title = Remettre { $file } ?
asset-rule-put-back-body = Le fichier converti est supprimé et l’original reprend sa place, avec ses étiquettes, ses collections et ses liens. Les règles de conversion n’y touchent plus ensuite.
asset-rule-put-back-done = { $file } est de retour
asset-rule-let-go = L’original a été supprimé une fois écoulée la durée de conservation des originaux fixée dans Paramètres.
asset-unreal-package = Du paquet Unreal
asset-ue-needs-editor-before = En attente d’Unreal Editor : { $reason }. Sur une machine où Unreal est installé, exécutez
asset-ue-needs-editor-after = et il revient en FBX.
asset-ue-editor-path = chemin vers UnrealEditor-Cmd
asset-ue-unsupported = Conservé en tant que paquet : { $reason }.
asset-ue-failed = Échec de la conversion : { $reason }
asset-recovered = Le .{ $ext } d’origine était stocké dans ce paquet, et Hoard l’en a extrait.
asset-recovered-download = Télécharger le .{ $ext }
asset-ue-meshes-before = Les maillages et les animations sont stockés dans le format propre à Unreal. Convertissez le pack (page du pack, Plus), puis exécutez
asset-ue-meshes-after = à côté d’une installation d’Unreal Editor pour obtenir du FBX.
asset-ue-registry-tags =
    { $count ->
        [one] { $count } tag du registre des ressources
       *[other] { $count } tags du registre des ressources
    }
asset-file = Fichier
asset-bytes = Octets
asset-source = Source
asset-path = Chemin
asset-path-open-folder = Voir ce qu’il y a d’autre dans { $folder }
asset-download-maps =
    { $count ->
        [one] Télécharger la { $count } carte
       *[other] Télécharger les { $count } cartes
    }
asset-download-linked =
    { $count ->
        [one] Télécharger avec { $count } fichier lié
       *[other] Télécharger avec { $count } fichiers liés
    }
asset-download-only-title = Seulement ce fichier, sans les fichiers qu’il utilise
asset-download-only = Ce fichier seulement
asset-include-animations = Inclure les animations dans le .zip
asset-include-character = Inclure le personnage { $name }
asset-copy-toml = Copier l’entrée hoard.toml
asset-copy-id = Copier l’ID
asset-teach = Apprendre ce fichier à Hoard
asset-teach-title = Décrire ce fichier de boutique pour que Hoard le lise (Paramètres → Fichiers des boutiques)
asset-archive = Archiver
asset-archive-title = Hors des recherches jusqu’à sa restauration ou sa suppression définitive, depuis les Archives

## ImageViewer.vue

image-channel-red = Canal rouge
image-channel-green = Canal vert
image-channel-blue = Canal bleu
image-channel-alpha = Canal alpha
image-channel-title = { $channel } ({ $key })
image-bg-checker-title = Fond en damier (K pour changer)
image-bg-black-title = Fond noir (K pour changer)
image-bg-gray-title = Fond gris (K pour changer)
image-bg-white-title = Fond blanc (K pour changer)
image-bg-checker = fond en damier
image-bg-black = fond noir
image-bg-gray = fond gris
image-bg-white = fond blanc
image-inspect-aria = Inspecter { $name }
image-preview-size = aperçu { $width }×{ $height }, { $bytes }
image-preview = aperçu
image-resolution = Résolution
image-full-title = Charger l’image complète en { $width }×{ $height } (F) ; zoomer au-delà de l’aperçu le fait aussi
image-full-loading = Chargement de la pleine résolution…
image-full-failed = La pleine résolution n’a pas chargé : réessayer
image-full = Pleine résolution
image-zoom = Zoom
image-zoom-out-title = Zoom arrière (−)
image-zoom-out = Zoom arrière
image-zoom-in-title = Zoom avant (+)
image-zoom-in = Zoom avant
image-fit-title = Ajuster (0)
image-fit = Ajuster
image-actual-title = Taille réelle (1)
image-view = Affichage
image-tile-title = Répéter en 3×3 pour vérifier les raccords (T)
image-tile = Répéter
image-channels = Canaux
image-close = Fermer (Esc)
image-failed = Cette image n’a pas pu être chargée.
image-loading = Chargement…
image-key-gestures = Défilez ou pincez pour zoomer · faites glisser pour vous déplacer · double-cliquez pour zoomer ou ajuster
image-key-tile = répéter
image-key-background = arrière-plan
image-key-full = pleine résolution
image-key-channels = canaux

## SpritePlayer.vue

sprite-frame-aria = { $name }, image { $frame } sur { $count }
sprite-failed = Celui-ci ne peut pas être lu.
sprite-loading = Chargement des images…
sprite-previous = Image précédente
sprite-next = Image suivante
sprite-pause = Mettre l’animation en pause
sprite-play = Lire l’animation
sprite-animation = Animation
sprite-all-frames = Toutes les images
sprite-grid-title = Comment la planche est découpée en images
sprite-columns = Colonnes
sprite-rows = Lignes
sprite-fps-aria = Images par seconde
sprite-fps = i/s

## La visionneuse de planches de sprites (SheetView.vue) et ses formats de téléchargement (AssetDetail.vue)

# Counts under the sheet's preview, joined with " · ".
sheet-sprites =
    { $count ->
        [one] { $count } sprite
       *[other] { $count } sprites
    }
sheet-pages =
    { $count ->
        [one] { $count } page
       *[other] { $count } pages
    }
sheet-animations =
    { $count ->
        [one] { $count } animation
       *[other] { $count } animations
    }
# The box that narrows the list of sprites by name.
sheet-find = Chercher un sprite
# Under the list of sprites when more match than it draws.
sheet-showing = { $shown } affichés sur { $total }
sheet-no-match = Aucun sprite n’a cela dans son nom.
sheet-failed = Cette planche ne peut pas être lue.
sheet-loading = Chargement de la planche…
# Under "Download as" when a sprite sheet format is picked: what the format can say, and so how the sheet comes out
# in it. Sprites are turned to fit more on a page only in formats that can say so.
sheet-note-hoard = Le format propre à Hoard : les pages telles quelles, les sprites pivotés dans n’importe quel sens et rognés, avec les animations et la durée de chaque image.
sheet-note-texturepacker = Sprites pivotés dans le sens horaire et rognés, avec les animations par nom, comme PixiJS et Phaser les lisent.
sheet-note-clockwise = Sprites pivotés dans le sens horaire et rognés. Il ne peut pas contenir d’animations : les images se suivent selon leurs noms numérotés.
sheet-note-libgdx = Sprites pivotés dans le sens antihoraire et rognés. Il ne peut pas contenir d’animations : les images se suivent selon leurs noms numérotés.
sheet-note-godot-frames = Sprites jamais pivotés, leur rognage conservé sous forme de marges, avec les animations et la durée de chaque image.
sheet-note-godot-tileset = Tuiles jamais pivotées ni rognées, sur une grille à la taille des tuiles, en gardant les images telles quelles si elles y sont déjà.
sheet-note-grid = Chaque sprite à l’endroit sur toute sa toile, dans des cellules toutes de la même taille, pour tout ce qui découpe une planche en grille.
sheet-note-frames = Chaque sprite à l’endroit sur toute sa toile, chacun dans un PNG qui porte son nom.

## TextViewer.vue, TextPreview.vue et text.ts

text-lang-config = Configuration
text-lang-table = Tableau
text-lang-code = Code
text-lang-script = Script
text-lang-text = Texte
text-copy-blocked = Copie impossible : le navigateur l’a bloquée
text-read-aria = Lire { $name }
text-lines =
    { $count ->
        [one] { $count } ligne
       *[other] { $count } lignes
    }
text-show-as = Afficher en
text-tree-title = Arbre repliable (T)
text-tree = Arbre
text-pretty-title = Réindenté (P)
text-pretty = Formaté
text-raw-title = Tel qu’écrit (R)
text-raw = Brut
text-document-title = En tant que document (D)
text-document = Document
text-source-title = Le markdown tel qu’écrit (R)
text-source = Source
text-page-title = Comme un navigateur l’affiche depuis son dossier, à l’écart de Hoard (P)
text-page = Page
text-page-source-title = La page telle qu’elle est écrite (R)
text-page-aria = { $name } en page
text-page-error = Impossible d’ouvrir cette page : { $error }
text-fold = Replier
text-expand-all-title = Tout déplier
text-expand-all = Tout déplier
text-collapse-all-title = Tout replier
text-collapse-all = Tout replier
text-group-text = Texte
text-wrap-title = Renvoyer les longues lignes à la ligne (W)
text-wrap = Retour à la ligne
text-copy-title = Copier le texte (C)
text-copied = Copié
text-copy = Copier
text-download-title = Télécharger le fichier
text-download = Télécharger
text-close = Fermer (Esc)
text-truncated = Affichage des premiers { $shown } sur { $total }.
text-truncated-json = Affichage des premiers { $shown } sur { $total } (l’arbre a besoin du fichier entier).
text-show-all = Tout afficher
text-json-error = Ce JSON ne se lit pas : { $error }
text-contents-aria = Contenu de { $name }
text-loading = Chargement…
text-load-error = Impossible de charger ce fichier : { $error }
text-binary = Ce fichier est binaire, pas du texte : il n’y a rien à lire ici. Téléchargez-le pour l’ouvrir dans l’outil adapté.
text-key-tree = arbre
text-key-pretty = formaté
text-key-raw = brut
text-key-document = document
text-key-source = source
text-key-page = page
text-key-wrap = retour à la ligne
text-key-copy = copier
text-key-find = rechercher
text-key-close = fermer
text-preview-aria = Lire { $name } en plein écran
text-preview-read = Lire

## JsonTree.vue et JsonPicker.vue

json-items =
    { $count ->
        [one] { $count } élément
       *[other] { $count } éléments
    }
json-keys =
    { $count ->
        [one] { $count } clé
       *[other] { $count } clés
    }
json-fold = Replier
json-unfold = Déplier
json-unfold-summary = Déplier { $summary }
json-show-more = Afficher { $count } de plus
json-left = sur { $count } restants
json-picker-more = … { $count } de plus

## Visionneuses de modèles 3D (ModelInspector.vue, ModelViewer.vue, modelStage.ts)

model-viewer-failed = la visionneuse 3D n’a pas chargé ({ $error }) ; recharger la page peut aider
model-loading-viewer = Chargement de la visionneuse 3D…
model-loading = Chargement du modèle…
model-show-failed = Impossible d’afficher ce modèle : { $error }
model-preview-failed = Impossible de prévisualiser ce modèle : { $error }
model-error-converting = le serveur est encore en train de convertir ce modèle
model-error-converting-desktop = Hoard est encore en train de convertir ce modèle
model-error-empty-collada = fichier Collada vide
model-error-no-loader = Aucun chargeur de navigateur pour .{ $ext }
model-dialog = Voir { $name } en 3D
model-title-stats =
    { $triangles } tris · { $meshes ->
        [one] { $meshes } maillage
       *[other] { $meshes } maillages
    } · { $materials ->
        [one] { $materials } matériau
       *[other] { $materials } matériaux
    }
model-triangles =
    { $count ->
        [one] { $count } triangle
       *[other] { $count } triangles
    }
model-group-camera = Caméra
model-group-shading = Ombrage
model-group-background = Arrière-plan
model-group-animation = Animation
model-group-view = Affichage
model-group-show = Afficher
model-frame-title = Cadrer le modèle (F)
model-frame = Cadrer
model-reset-title = Revenir à la vue de départ (R)
model-reset = Réinitialiser
model-turn-title = Tourner lentement (A)
model-turn = Tourner
model-textures-title = Textures (T)
model-textures = Textures
model-full-res-title = Textures en pleine résolution (H) : chaque fichier de texture tel quel, quelle que soit sa taille ; désactivé, des aperçus légers de 2048 px au plus
model-full-res = Pleine résolution
model-light-title = Éclairage d’environnement (E)
model-light = Lumière
model-grid-title = Grille au sol (G)
model-grid = Grille
model-bg-checker = Fond en damier
model-bg-dark = Fond sombre
model-bg-gray = Fond gris
model-bg-light = Fond clair
model-bg-env = Fond d’environnement
model-bg-title = { $background } (K pour changer)
model-pause-title = Pause (Espace)
model-play-title = Lecture (Espace)
model-pause = Pause
model-play = Lecture
model-animation-clip = Clip d’animation
model-animation = Animation
model-plays-on = Jouée sur
model-rest-pose = Pose de repos
model-close-title = Fermer (Esc)
model-full-screen = Plein écran
model-open-full-screen = Ouvrir la visionneuse 3D en plein écran
model-menu-fold = Replier le menu
model-menu-show = Afficher le menu
model-view-prefix = Vue :{ " " }
model-menu-render = Rendu
model-menu-maps = Cartes
model-menu-inspect = Inspecter
model-mode-final = Rendu final
model-mode-raw = Rendu brut
model-mode-color = Couleur de base
model-mode-roughness = Rugosité
model-mode-metalness = Métallicité
model-mode-normal = Normales
model-mode-ao = Occlusion
model-mode-matcap = Matcap
model-mode-uv = Damier UV
model-mode-geometry = Géométries
model-mode-title = { $mode } ({ $key })
model-wireframe-title = Fil de fer (W), par-dessus n’importe quelle vue
model-wireframe = Fil de fer
model-info-title = Informations sur la scène (I)
model-info = Informations sur la scène
model-info-close = Fermer les informations sur la scène
model-info-close-title = Fermer (I)
model-info-geometries = Géométries
model-info-materials = Matériaux
model-info-textures = Textures
model-info-triangles = Triangles
model-info-vertices = Sommets
model-info-meters = Le glTF est en mètres
model-info-no-units = Ce format n’indique pas ses unités
model-info-height = Hauteur
model-info-length = Longueur
model-info-width = Largeur
model-length-units =
    { $n ->
        [one] { NUMBER($n, maximumSignificantDigits: 3) } unité
       *[other] { NUMBER($n, maximumSignificantDigits: 3) } unités
    }
model-length-m = { NUMBER($n, maximumSignificantDigits: 3) } m
model-length-cm = { NUMBER($n, maximumSignificantDigits: 3) } cm
model-length-mm = { NUMBER($n, maximumSignificantDigits: 3) } mm
model-missing-one = Texture introuvable : { $name }
model-missing-some = { $count } textures introuvables : { $names }
model-missing-more = { $count } textures introuvables : { $names }…
model-missing-count = { $count } textures introuvables
model-guessed = Textures associées d’après leur nom ; le fichier n’en nomme aucune.
model-group-facing = Orientation
model-facing-turn = Tourner d’un quart
model-facing-turn-title = Faire tourner le modèle d’un quart de tour dans le sens antihoraire, vu de dessus
model-facing-tip = Basculer d’un quart
model-facing-tip-title = Basculer le haut du modèle d’un quart de tour vers l’avant, pour redresser un modèle couché
model-facing-save = Appliquer au pack
model-facing-save-title = Tourner ainsi tous les modèles de { $pack }. Leurs miniatures sont redessinées ; les téléchargements restent tels qu’ils ont été faits.
model-facing-undo = Annuler
model-facing-undo-title = Revenir à l’orientation actuelle des modèles du pack
model-facing-saved = Tous les modèles de { $pack } sont désormais orientés ainsi. Leurs miniatures sont en train d’être redessinées.
model-facing-failed = Impossible de tourner les modèles du pack : { $error }
model-keys-mouse = Faites glisser pour orbiter · clic droit et glisser pour vous déplacer · défilez pour zoomer · double-cliquez pour cadrer
model-key-frame = cadrer
model-key-reset = réinitialiser
model-key-view = vue
model-key-wireframe = fil de fer
model-key-info = informations sur la scène
model-key-textures = textures
model-key-full-res = textures en pleine résolution
model-key-light = lumière
model-key-grid = grille
model-key-background = arrière-plan
model-key-play = lecture

## Matériaux (MaterialViewer.vue, MaterialPreview.vue, materials.ts)

material-role-base-color = Couleur
material-role-normal = Normale
material-role-normal-dx = Normale (DirectX)
material-role-roughness = Rugosité
material-role-gloss = Brillance
material-role-metalness = Métallicité
material-role-ao = Occlusion ambiante
material-role-orm = Occlusion, rugosité, métal
material-role-mra = Métal, rugosité, occlusion
material-role-rma = Rugosité, métal, occlusion
material-role-height = Hauteur
material-role-bump = Relief
material-role-cavity = Cavité
material-role-specular = Spéculaire
material-role-opacity = Opacité
material-role-emissive = Émission
material-role-translucency = Translucidité
material-role-base-color-name = couleur
material-role-normal-name = normale
material-role-normal-dx-name = normale (DirectX)
material-role-roughness-name = rugosité
material-role-gloss-name = brillance
material-role-metalness-name = métallicité
material-role-ao-name = occlusion ambiante
material-role-orm-name = occlusion, rugosité, métal
material-role-mra-name = métal, rugosité, occlusion
material-role-rma-name = rugosité, métal, occlusion
material-role-height-name = hauteur
material-role-bump-name = relief
material-role-cavity-name = cavité
material-role-specular-name = spéculaire
material-role-opacity-name = opacité
material-role-emissive-name = émission
material-role-translucency-name = translucidité
material-maps =
    { $count ->
        [one] { $count } carte
       *[other] { $count } cartes
    }
material-dialog = Voir le matériau { $name }
material-up-to = jusqu’à { $size }
material-repeat-size = { $size } par répétition
material-shape-sphere = Sphère
material-shape-cube = Cube
material-shape-plane = Plan
material-shape-title = { $shape } ({ $key })
material-group-shape = Forme
material-group-tiling = Répétition
material-real-title = Taille réelle : une répétition fait { $size }, la sphère 2 m de diamètre (T pour changer)
material-repeat-title = Répéter { $n }× (T pour changer)
material-real = Réelle
material-repeat = { $n }×
material-height-title = À quel point la carte de hauteur déforme la surface
material-height = Hauteur
material-height-strength = Intensité de la hauteur
material-loading = Chargement du matériau…
material-show-failed = Impossible d’afficher ce matériau : { $error }
material-preview-failed = Impossible de prévisualiser ce matériau : { $error }
material-group-maps = Cartes
material-map-show = Afficher la carte { $map }
material-map-hide = Masquer la carte { $map }
material-map-unused = Non utilisée par l’aperçu
material-open-full-screen = Ouvrir la visionneuse de matériaux en plein écran
material-keys-mouse = Faites glisser pour orbiter · clic droit et glisser pour vous déplacer · défilez pour zoomer · cliquez sur une carte pour la désactiver
material-key-shape = forme
material-key-tiling = répétition
material-key-turn = tourner

## theme.ts

theme-slate = Ardoise
theme-paper = Papier
theme-linen = Lin
theme-sage = Sauge
theme-lilac = Lilas
theme-blush = Rose poudré
theme-graphite = Graphite
theme-obsidian = Obsidienne
theme-ember = Braise
theme-moss = Mousse
theme-aubergine = Aubergine
theme-oxblood = Sang-de-bœuf
theme-light-slate-blurb = Gris-bleu froid
theme-light-paper-blurb = Gris neutre et simple
theme-light-linen-blurb = Crème chaud, ambre
theme-light-sage-blurb = Vert doux
theme-light-lilac-blurb = Violet pâle
theme-light-blush-blurb = Vieux rose
theme-dark-graphite-blurb = Gris neutre, sans teinte
theme-dark-slate-blurb = Gris-bleu
theme-dark-obsidian-blurb = Presque noir, pour les écrans OLED
theme-dark-ember-blurb = Anthracite chaud, ambre
theme-dark-moss-blurb = Noir vert forêt
theme-dark-aubergine-blurb = Noir prune, lilas
theme-dark-oxblood-blurb = Noir lie-de-vin, corail

## texnames.ts

texnames-needs-map = Le modèle doit contenir { "{" }map{ "}" }, sinon toutes les cartes d’un matériau porteraient le même nom
texnames-placeholders = Le modèle peut utiliser { "{" }name{ "}" }, { "{" }map{ "}" } et { "{" }size{ "}" }
texnames-bad-characters = Les noms ne peuvent pas contenir < > : " / \ | ? ou *
texnames-no-suffix = Donnez un suffixe aux cartes { $role }
texnames-bad-suffix = Le suffixe des cartes { $role } ne peut pas contenir < > : " / \ | ? ou *
texnames-slot-no-suffix = Donnez un suffixe à « { $slot } »
texnames-slot-bad-suffix = Le suffixe de « { $slot } » ne peut pas contenir < > : " / \ | ? ou *
texnames-ending-missing = Chaque terminaison personnalisée doit indiquer la terminaison à rechercher
texnames-ending-bad = Les terminaisons personnalisées ne peuvent pas contenir < > : " / \ | ? ou *
texnames-ending-twice = { $ending } figure deux fois dans vos terminaisons
texnames-slot-metallic-roughness = Métal et rugosité
texnames-slot-specular-color = Couleur spéculaire
texnames-slot-specular-glossiness = Spéculaire et brillance
texnames-slot-clearcoat = Vernis
texnames-slot-clearcoat-roughness = Rugosité du vernis
texnames-slot-clearcoat-normal = Normale du vernis
texnames-slot-sheen-color = Couleur du lustre
texnames-slot-sheen-roughness = Rugosité du lustre
texnames-slot-transmission = Transmission
texnames-slot-thickness = Épaisseur
texnames-slot-diffuse-transmission = Transmission diffuse
texnames-slot-diffuse-transmission-color = Couleur de transmission diffuse
texnames-slot-iridescence = Irisation
texnames-slot-iridescence-thickness = Épaisseur d’irisation
texnames-slot-anisotropy = Anisotropie

## storeFormats.ts

storefiles-field-title = Titre
storefiles-field-title-hint = Le titre exact de l’annonce. Il nomme le pack ; sans lui, le fichier est ignoré.
storefiles-field-seller = Vendeur
storefiles-field-seller-hint = Qui l’a créé ou le vend.
storefiles-field-license = Licence
storefiles-field-license-hint = Telle que le fichier l’écrit, par exemple CC-BY-4.0.
storefiles-field-url = Page de boutique
storefiles-field-url-hint = L’adresse de l’annonce, commençant par http:// ou https://.
storefiles-field-words = Étiquettes
storefiles-field-words-hint = Les mots avec lesquels l’étiqueter. Les valeurs séparées par des virgules sont découpées.
storefiles-field-type = Type
storefiles-field-type-hint = Ce que c’est, par exemple « 3D model » ou « surface ». C’est aussi une étiquette.
storefiles-field-colors = Couleurs
storefiles-field-colors-hint = Ses couleurs, sous forme d’étiquettes color:.
storefiles-field-id = ID de boutique
storefiles-field-id-hint = L’identifiant que la boutique lui donne : lettres, chiffres, _ ou -.
storefiles-field-origin = Lieu de création
storefiles-field-origin-hint = Où il a été créé ou scanné, pour la description.
storefiles-field-size = Taille
storefiles-field-size-hint = Sa taille, en toutes lettres, pour la description.
storefiles-field-title-name = le titre
storefiles-field-seller-name = le vendeur
storefiles-field-license-name = la licence
storefiles-field-url-name = la page de boutique
storefiles-field-words-name = les étiquettes
storefiles-field-type-name = le type
storefiles-field-colors-name = les couleurs
storefiles-field-id-name = l’ID de boutique
storefiles-field-origin-name = le lieu de création
storefiles-field-size-name = la taille

## StoreFormatsPanel.vue

storefiles-load-failed = Impossible de charger les formats de fichiers de boutique : { $error }
storefiles-remove-title = Supprimer { $name } ?
storefiles-remove-body = { $file } est supprimé du dossier de données. Les packs déjà lus avec lui gardent ce qu’il en a lu.
storefiles-remove = Supprimer
storefiles-removed = { $name } supprimé.
storefiles-remove-failed = Impossible de supprimer { $name } : { $error }
storefiles-unread = Fichiers que Hoard ne lit pas
storefiles-loading = Chargement…
storefiles-unread-none = Aucun : aucun fichier de la bibliothèque ne ressemble à celui d’une boutique non lue.
storefiles-unread-what = Fichiers présents dans plusieurs packs, ou nommés comme ceux d’une boutique, qu’aucun format ne lit.
storefiles-unread-packs =
    { $count ->
        [one] { $count } pack
       *[other] { $count } packs
    }
storefiles-unread-example = par ex. { $pack }
storefiles-teach = Apprendre ce fichier à Hoard
storefiles-formats = Formats
storefiles-formats-what = Les vôtres sont essayés en premier et remplacent un format intégré du même nom.
storefiles-dir-before = Ils sont conservés dans
storefiles-dir-after = .
storefiles-yours = Le vôtre
storefiles-built-in = Intégré
storefiles-edit = Modifier
storefiles-start-from = Partir de celui-ci

## TagsPanel.vue

tags-load-failed = Impossible de charger les étiquettes : { $error }
tags-rename-title = Renommer { $tag } ?
tags-merge-title = Fusionner { $count } étiquettes ?
tags-rename-body =
    { $count ->
        [one] Sa { $count } ressource reçoit
       *[other] Ses { $count } ressources reçoivent
    } { $into } à la place, et les étiqueteurs utilisent désormais le nouveau nom.
tags-merge-body = { $tags } deviennent { $into } sur { $count } ressources au plus. Les étiqueteurs utilisent désormais { $into }.
tags-rename = Renommer
tags-merge = Fusionner
tags-merged =
    { $count ->
        [one] { $count } ressource désormais étiquetée { $into }
       *[other] { $count } ressources désormais étiquetées { $into }
    }
tags-merge-failed = Impossible de fusionner : { $error }
tags-forget-failed = Impossible de modifier cela : { $error }
tags-find = Trouver des étiquettes
tags-find-placeholder = Trouvez des étiquettes, par ex. rock ou set:
tags-sort = Ordre
tags-sort-count = Les plus utilisés
tags-sort-name = A–Z
tags-count =
    { $count ->
        [one] { $count } étiquette
       *[other] { $count } étiquettes
    }
tags-count-more = { $count }+ étiquettes
tags-what = Choisissez des étiquettes qui veulent dire la même chose et fusionnez-les, ou choisissez-en une pour la renommer.
tags-new-name = Nouveau nom
tags-merge-into = Fusionner en
tags-saved-as-before = Enregistrée sous
tags-saved-as-after = { "" }
tags-cancel = Annuler
tags-loading = Chargement…
tags-none = Aucune étiquette.
tags-none-containing = Aucune étiquette contenant « { $text } ».
tags-browse-title = Parcourir les ressources étiquetées { $tag }
tags-browse = Parcourir
tags-merged-heading = Étiquettes fusionnées
tags-merged-what = Quand les étiqueteurs proposent l’un de ces anciens noms, la ressource reçoit l’étiquette dans laquelle il a été fusionné.
tags-allow-old = Autoriser de nouveau l’ancien nom
# A tag's own page, above the results while that one tag is browsed. How many assets have it.
tag-page-count =
    { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }
# Old names merged into the tag ($names, separated by commas).
tag-page-aliases =
    { $count ->
        [one] Ancien nom : { $names }
       *[other] Anciens noms : { $names }
    }
# Browsing a tag that was merged into another.
tag-page-merged = { $tag } a été fusionnée en { $into }.
tag-page-show = Afficher { $tag }
# Shown to people who edit when the tag says nothing yet; clicking it lets them write what it means.
tag-page-describe = Dites ce que signifie cette étiquette…
tag-page-describe-label = Ce que signifie cette étiquette
tag-page-describe-placeholder = À quoi elle sert, ce qui en relève. Entrée enregistre.
tag-page-describe-edit = Cliquez pour modifier
tag-page-description-saved = Signification de l'étiquette enregistrée
# The tag's picture: one of its assets stands for it.
tag-page-picture = Choisir une image pour cette étiquette
tag-page-picture-help = Choisissez une de ses ressources pour la représenter
tag-page-picture-nothing = Aucune des ressources affichées n'a d'image.
tag-page-picture-none = Aucune image
tag-page-picture-saved = Image définie
tag-page-picture-removed = Image retirée
tag-page-rename = Renommer…
tag-page-rename-title = Donnez-lui un nouveau nom, ou le nom d'une autre étiquette pour la fusionner avec celle-ci
tag-page-rename-label = Nouveau nom
tag-page-save-failed = Impossible d'enregistrer l'étiquette : { $error }
# Above a tag's name on its page: the broader tags it goes under, each a link.
tag-page-under = Sous
tag-page-parent-add = Placer sous…
tag-page-parent-add-title = Placez cette étiquette sous une plus large : en cherchant la plus large, vous trouverez aussi celles-ci
tag-page-parent-placeholder = Une étiquette plus large, comme véhicule
# The × beside a broader tag on a tag's page.
tag-page-parent-remove = Retirer { $tag } de sous { $parent }
tag-page-parent-saved = { $tag } est maintenant sous { $parent }
tag-page-parent-removed = { $tag } n'est plus sous { $parent }
# Before the narrower tags on a tag's page, each a link with how many assets have it.
tag-page-narrower = Plus précises :
tag-page-narrower-title = Chercher { $tag } trouve aussi celles-ci
# Settings → Tags: the tags put under broader ones, as a tree.
tags-tree-heading = Plus larges et plus précises
tags-tree-what = Étiquettes placées sous des plus larges depuis leur page. Chercher une étiquette trouve aussi tout ce qui est dessous.

## LibraryStats.vue

stats-source-human = Ajoutées par des personnes
stats-source-ai = Suggérées par l’IA
stats-source-rule = Règles intégrées
stats-source-meta = Tirées des fichiers
stats-assets = Ressources
stats-packs = Packs
stats-on-disk = Sur le disque
stats-duplicates-title = { $count } fichiers sont des copies identiques octet pour octet d’autres fichiers et ne sont stockés qu’une fois
stats-saved-dedup = Économisé par la déduplication
stats-distinct-tags = Étiquettes distinctes
stats-archive = Dans les archives
stats-load-failed = Impossible de charger les détails : { $error }
stats-counting = Décompte…
stats-as-of = Au { $time }
stats-updating = Mise à jour…
stats-by-kind = Espace par type
stats-files-size = { $name } : { $count } fichiers, { $size }
stats-months = Ajoutées au cours de la dernière année
stats-month =
    { $month } : { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }, { $size }
stats-tagging = Étiquetage
stats-tagged = Étiquetées
stats-untagged = { $count } sans étiquette
stats-described = Décrites
stats-described-assets =
    { $count ->
        [one] { $count } ressource
       *[other] { $count } ressources
    }
stats-human-edited = Modifiées à la main
stats-human-edited-unit =
    { $count ->
        [one] ressource
       *[other] ressources
    }
stats-source-different = { $count } distinctes
stats-most-used = Les plus utilisées
stats-show-tag = Afficher les ressources { $tag }
stats-file-types = Types de fichiers
stats-biggest-packs = Plus gros packs
stats-browse-pack = Parcourir { $name }
stats-largest-files = Plus gros fichiers
stats-open-file = Ouvrir { $name }
stats-empty = Rien dans la bibliothèque pour l’instant. Ajoutez un pack et ses chiffres apparaîtront ici.

## HoardPanel.vue

hoard-title = Votre trésor
hoard-what = Chaque ressource de la bibliothèque est une pièce du trésor, et son type décide laquelle. Ramassez les nouveautés pour les ajouter à votre tas.
hoard-sign-in = Connectez-vous pour amasser votre propre trésor.
hoard-new = Nouveau butin à ramasser : { $count }
hoard-none-new = Rien de nouveau à ramasser.
hoard-collect = Ramasser
hoard-unavailable = Ce navigateur ne peut pas dessiner le trésor : { $error }
hoard-collect-failed = Impossible d’enregistrer ce que vous avez ramassé : { $error }
hoard-arcade = Écran d’arcade
hoard-full-screen = Plein écran
hoard-leave-full-screen = Quitter le plein écran
hoard-label = Un dragon qui garde son trésor. Pièces : { $count }
hoard-gold = Or
hoard-gems = Gemmes
hoard-books = Livres
hoard-weapons = Armes
hoard-bones = Ossements
hoard-tablets = Tablettes runiques
hoard-paintings = Tableaux
hoard-potions = Potions

## UpdatesPanel.vue

updates-saved = Hoard l’appellera quand une mise à jour sera prête
updates-saved-none = Aucune adresse à appeler
updates-save-failed = Impossible d’enregistrer
updates-call-answered = Elle a répondu { $result }
updates-call-refused = Elle n’a pas accepté la requête : { $result }
updates-call-failed = Impossible de l’appeler
updates-called = Appelée le { $when } pour { $version } : { $result }
updates-called-failed = Appelée le { $when } pour { $version } : { $result }. Hoard réessaie dans une heure.
updates-latest = Hoard { $version } est la dernière version.
updates-out = Hoard { $version } est sorti.
updates-out-idle = Hoard { $version } est sorti ; appel d’ici quelques minutes, dès que rien n’est en cours.
updates-out-busy = Hoard { $version } est sorti ; en attente que plus rien ne soit en cours ({ $work }).
updates-busy-more = { $count } de plus
updates-busy-jobs =
    { $count ->
        [one] { $count } tâche en attente
       *[other] { $count } tâches en attente
    }
updates-busy-uploads =
    { $count ->
        [one] { $count } envoi
       *[other] { $count } envois
    }
updates-list-separator = ,{ " " }
updates-what = Quand une nouvelle version sort, Hoard peut appeler une adresse dès que rien n’est en cours (ni envoi, ni tâche, ni conversion, ni maintenance), pour que ce qui met à jour son conteneur ne le redémarre pas en plein envoi. Il envoie un POST avec la version dans un corps JSON. Cela nécessite la vérification des nouvelles versions ci-dessus.
updates-url = Adresse à appeler
updates-url-placeholder = Aucune (p. ex. http://watchtower:8080/v1/update?container=hoard&async=true)
updates-auth = Authorization
updates-auth-saved = Enregistrée (tapez pour la remplacer)
updates-auth-placeholder = Aucune (p. ex. Bearer my-token)
updates-remove = Retirer
updates-save = Enregistrer
updates-call-now-title = Ce qui y est branché s’exécute
updates-call-now = L’appeler maintenant
updates-what-calls = Que peut-elle appeler ?
updates-watchtower = L’API HTTP de Watchtower
updates-watchtower-at = { " " }:{ " " }
updates-watchtower-with = , avec{ " " }
updates-watchtower-token = { " " }et le jeton de{ " " }
updates-watchtower-its = { " " }(son point d’accès{ " " }
updates-watchtower-endpoint = { " " }activé).
updates-portainer = Un webhook Portainer
updates-portainer-rest = { " " }sur la pile ou le conteneur de Hoard, avec « re-pull image » activé. Le jeton figure dans l’URL.
updates-script = Un script sur l’hôte
updates-script-listener = { " " }derrière un écouteur de webhooks (comme{ " " }
updates-script-runs = ) qui exécute{ " " }
updates-script-end = .
updates-others = Home Assistant, n8n
updates-others-rest = { " " }ou tout autre outil qui accepte un webhook.
updates-poll-before = Ou passez-vous d’adresse : un script muni d’un jeton d’accès peut interroger{ " " }
updates-poll-until = { " " }jusqu’à ce que{ " " }
updates-poll-and = { " " }soit renseigné et que{ " " }
updates-poll-after = { " " }vaille true, puis récupérer la nouvelle version et redémarrer Hoard.

## MaintenancePanel.vue

maintenance-load-failed = Impossible de charger la maintenance
maintenance-reanalyze-title = Réanalyser tous les packs ?
maintenance-reanalyze-body = Chaque fichier est relu, ce qui peut prendre longtemps sur une grosse bibliothèque ; les packs apparaissent en cours de traitement jusqu’à leur tour. Vos propres noms, étiquettes et notes sont conservés.
maintenance-reanalyze-confirm = Réanalyser
maintenance-gc-title = Supprimer les fichiers inutilisés ?
maintenance-gc-body = Les fichiers stockés auxquels rien dans la bibliothèque ne fait référence sont définitivement supprimés. Les packs des archives gardent les leurs.
maintenance-gc-confirm = Les supprimer
maintenance-start-failed = Impossible de lancer « { $task } »
maintenance-just-now = à l’instant
maintenance-minutes-ago = il y a { $count } min
maintenance-hours-ago = il y a { $count } h
maintenance-took-seconds = { $count } s
maintenance-took-minutes = { $count } min
maintenance-took-hours = { $hours } h
maintenance-loading = Chargement…
maintenance-running = En cours…
maintenance-starting = Démarrage…
maintenance-run = Lancer
maintenance-stop = Arrêter
maintenance-stopping = Arrêt en cours…
maintenance-stop-failed = Impossible d’arrêter « { $task } »
maintenance-started = { $detail } · lancée { $when }
maintenance-starting-detail = Démarrage
maintenance-finished = Terminé :{ " " }
maintenance-failed = Échec :{ " " }
maintenance-never-run = Jamais lancée
maintenance-history = Exécutions passées
maintenance-col-task = Tâche
maintenance-col-finished = Terminée
maintenance-col-took = Durée
maintenance-col-result = Résultat
maintenance-script-before = Les sauvegardes et la vérification du stockage peuvent aussi être lancées depuis un script :{ " " }
maintenance-script-comma = ,{ " " }
maintenance-script-and = { " " }et{ " " }
maintenance-script-after = . Ces exécutions apparaissent ici aussi.

## SettingsPage.vue

settings-title = Paramètres
settings-loading = Chargement…
settings-find = Trouver un réglage
settings-find-none = Aucun réglage ne correspond à « { $query } ».
settings-load-failed = Impossible de charger les paramètres : { $error }
settings-page-appearance = Apparence
settings-page-appearance-blurb = Les thèmes que ce navigateur porte en mode clair et en mode sombre. Enregistrés dans ce navigateur uniquement.
settings-page-dragon = Dragon
settings-page-dragon-blurb = Le trésor de votre dragon : chaque ressource de la bibliothèque en est une pièce.
settings-page-library = Bibliothèque
settings-page-library-blurb = Ce que contient Hoard : sa taille, de quoi il est fait, comment il a grandi et à quel point il est bien étiqueté. Ce qui est archivé n’est pas compté.
settings-page-copies = Quasi-copies
settings-page-copies-blurb = Les fichiers qui sont la même image, le même son ou le même modèle dans une autre taille, fréquence d’échantillonnage ou un autre format, ceux qui prennent le plus de place d’abord. La meilleure de chaque groupe est signalée ; rien n’est supprimé.
copies-loading = Recherche des quasi-copies…
copies-refreshing = La bibliothèque a changé depuis. Nouvelle recherche…
copies-none = Aucune quasi-copie : chaque image, son et modèle n’est ici qu’une fois.
copies-load-failed = Impossible de lister les quasi-copies : { $error }
copies-sum = Groupes de quasi-copies : { $count }. Les copies qui ne sont pas les meilleures occupent { $size }.
copies-set = Groupe { $n }
copies-set-wasted = { $size } dans les copies qui ne sont pas les meilleures
copies-more = Afficher plus (encore { $count })
settings-page-tags = Étiquettes
settings-page-tags-blurb = Toutes les étiquettes de la bibliothèque. Fusionnez celles qui veulent dire la même chose, ou renommez-en une ; les étiqueteurs utilisent le nouveau nom à partir de là.
settings-page-store-files = Fichiers des boutiques
settings-page-store-files-blurb = Les fichiers que les boutiques placent à côté de leurs ressources, que Hoard lit pour nommer les packs et créditer leurs auteurs. Apprenez-lui ceux qu’il ne connaît pas, et partagez ce que vous lui avez appris.
settings-page-locations = Emplacements de la bibliothèque
settings-page-locations-blurb = Les dossiers du serveur qui contiennent la bibliothèque : ceux où Hoard range des fichiers, et les dossiers de packs qu’il indexe là où ils se trouvent.
settings-page-locations-blurb-desktop = Les dossiers qui contiennent la bibliothèque : ceux où Hoard range des fichiers, et les dossiers de packs qu’il indexe là où ils se trouvent.
settings-page-tools = Outils
settings-page-tools-blurb = Les programmes externes que Hoard utilise pour convertir et lire les fichiers.
settings-page-ai = Étiquetage par IA
settings-page-ai-blurb = Un modèle d’IA nomme, décrit et étiquette les nouveaux packs, en plus des règles intégrées. Ses étiquettes sont marquées comme suggérées.
settings-page-search-blurb = Cherchez des images d’après ce qu’elles montrent et des sons d’après ce à quoi ils ressemblent, pas seulement d’après leur nom, et trouvez des choses de même nature, pas seulement de même couleur.
settings-page-search-tab = Recherche
settings-page-adding = Ajout de packs
settings-page-adding-blurb = Ce que Hoard fait d’un pack quand il est ajouté, en plus de le lire et de l’étiqueter.
settings-page-downloads = Téléchargements
settings-page-downloads-blurb = Comment sont nommés les fichiers que Hoard envoie : téléchargements, zips, l’outil hoard et les assistants IA. Les fichiers de la bibliothèque restent tels quels.
settings-page-account = Compte
settings-page-account-blurb = Votre mot de passe, les autres personnes qui peuvent se connecter, et les jetons d’accès pour les scripts et autres programmes.
settings-page-auth = Authentification
settings-page-auth-blurb = Comment on se connecte : les comptes propres à Hoard, un annuaire LDAP, Microsoft Active Directory ou un fournisseur OAuth, la durée des sessions, et quels groupes obtiennent quel rôle.
settings-page-connection = Connexion
settings-page-connection-blurb = Comment les navigateurs et les applications joignent Hoard : l’adresse sous laquelle il est connu, et HTTPS, dont certaines fonctions des navigateurs ont besoin.
settings-page-performance = Performances
settings-page-performance-blurb = L’intensité de travail du serveur, la façon dont les miniatures sont créées et conservées, et le maximum qu’il accepte.
settings-page-performance-blurb-desktop = L’intensité de travail de Hoard, la façon dont les miniatures sont créées et conservées, et le maximum qu’il accepte.
settings-page-issues = Problèmes
settings-page-issues-blurb = Ce qui a mal tourné à l’ouverture ou au traitement des fichiers : archives impossibles à décompresser, fichiers illisibles, tâches en échec. Réessayez une fois la cause corrigée, ou ignorez-les.
settings-page-maintenance = Maintenance
settings-page-maintenance-blurb = Sauvegarder, vérifier l’intégrité et faire le ménage. Une tâche de chaque type s’exécute à la fois, et vous pouvez quitter cette page pendant ce temps.
settings-page-about = À propos
settings-page-about-blurb = Ce qu’est Hoard, de quelle version il s’agit et le travail d’autres personnes dont il est fait.
settings-the-default = la valeur par défaut
settings-default-on = activé
settings-default-off = désactivé
settings-default-auto = automatique
settings-option-saved = { $name } : { $value }
settings-back-to-default = Revenir à la valeur par défaut
settings-save = Enregistrer
settings-use = Utiliser
settings-status-off = Désactivé

## SettingsPage.vue : Apparence

settings-themes = Thèmes
settings-look-light = Clair
settings-look-dark = Sombre
settings-look-group =
    { $scheme ->
        [light] Thème clair
       *[dark] Thème sombre
    }
settings-preview =
    Affichage du thème { $preview ->
        [light] clair
       *[dark] sombre
    } pour que vous puissiez le voir. Vous utilisez le thème { $used ->
        [light] clair
       *[dark] sombre
    }{ $system ->
        [yes] { " " }(selon le système)
       *[other] { "" }
    }, qui revient quand vous quittez cette page.
settings-preview-back =
    Revenir au thème { $used ->
        [light] clair
       *[dark] sombre
    }
settings-look-hint = Passez du clair au sombre (ou suivez le système) depuis la barre du haut.

## SettingsPage.vue : Outils

settings-blender-what = Convertit en arrière-plan les modèles .blend, .dae, .abc et .x3d en glTF, pour qu’ils aient des miniatures et un aperçu 3D. Les packs ajoutés pendant qu’il était désactivé sont convertis dès qu’il est prêt.
settings-blender-download = Utilise un Blender installé, ou télécharge Blender 4.5 LTS (environ 400 Mo) depuis un miroir de blender.org.
settings-ffmpeg-what = Encode les téléchargements en MP3, OGG et Opus, et lit l’audio que Hoard ne sait pas décoder lui-même (AAC, M4A, WMA…).
settings-ffmpeg-download = Utilise un ffmpeg du système doté des bons encodeurs, ou télécharge une version statique (environ 80 Mo).
settings-tool-reset = { $tool } : retour à la valeur par défaut
settings-tool-no-download = Utilise un { $tool } installé. Il n’existe pas de version à télécharger pour cette machine : installez-le ou indiquez son chemin.
settings-tool-path = Chemin du programme { $tool }
settings-mode-off = Désactivé
settings-mode-auto = Automatique
settings-mode-path = Ce programme
settings-tool-looking = Recherche…
settings-tool-downloading = Téléchargement… la progression s’affiche dans l’indicateur d’activité
settings-tool-ready = Prêt : { $detail }
settings-soundfont = SoundFont MIDI
settings-soundfont-what = Les instruments avec lesquels les fichiers MIDI sont joués et convertis.
settings-soundfont-file = Fichier SoundFont
settings-soundfont-placeholder = Trouvé sur le serveur, ou intégré
settings-soundfont-placeholder-desktop = Trouvé sur cet ordinateur, ou intégré
settings-soundfont-default = trouvé sur le serveur, ou intégré
settings-soundfont-default-desktop = trouvé sur cet ordinateur, ou intégré
settings-soundfont-send = Choisir un fichier…
settings-soundfont-sent = Le MIDI joue maintenant avec { $name }

## SettingsPage.vue : Recherche d'images

settings-clip-heading = Chercher d’après ce que montrent les images
settings-clip-reset = Recherche d’images : retour à la valeur par défaut
settings-clip-saved-on = Recherche d’images : activée
settings-clip-saved-off = Recherche d’images : désactivée
settings-clip-what = Tapez ce que vous cherchez (« rocher moussu », « porte de science-fiction », « tonneau en bois ») et les images qui le montrent apparaissent à côté des noms qui contiennent ces mots. « Plus comme ceci » trouve des choses de même nature, pas seulement de mêmes couleurs.
settings-clip-toggle = Activer la recherche d’images
settings-clip-hint = Utilise le modèle SigLIP 2 de Google, exécuté sur le processeur de ce serveur : rien n’en sort. L’activer télécharge le modèle ({ $size } Mo) depuis Hugging Face, puis apprend chaque image en arrière-plan, en basse priorité. Il occupe de 350 à 750 Mo de mémoire pendant qu’il travaille ou que quelqu’un cherche. Le désactiver supprime le modèle et garde ce qu’il a appris, pour la prochaine fois.
settings-clip-hint-desktop = Utilise le modèle SigLIP 2 de Google, exécuté sur le processeur de cet ordinateur : rien n’en sort. L’activer télécharge le modèle ({ $size } Mo) depuis Hugging Face, puis apprend chaque image en arrière-plan, en basse priorité. Il occupe de 350 à 750 Mo de mémoire pendant qu’il travaille ou que vous cherchez. Le désactiver supprime le modèle et garde ce qu’il a appris, pour la prochaine fois.
settings-clip-looking = Recherche du modèle…
settings-clip-downloading = Téléchargement du modèle… la progression s’affiche dans l’indicateur d’activité
settings-clip-ready-empty = Prêt. Les images sont apprises à mesure que des packs sont ajoutés.
settings-clip-ready-all =
    { $count ->
        [one] Prêt : connaît { $count } image
       *[other] Prêt : connaît les { $count } images
    }
settings-clip-learning = Prêt : apprentissage des images, { $done } sur { $total } jusqu’ici
settings-clip-relearning = Tant que le nouveau modèle n’a pas appris toutes les images, les recherches utilisent ce que l’ancien a appris.
settings-clip-unavailable = Impossible d’obtenir le modèle : { $detail }. Nouvel essai toutes les heures.
settings-clip-where = S’exécute sur
settings-clip-here = Ce serveur : rien n’en sort
settings-clip-here-desktop = Cet ordinateur : rien n’en sort
settings-clip-hosted = Un service hébergé
settings-clip-hosted-warning = Les images quittent ce serveur. Une miniature de chaque image de la bibliothèque, et les mots de chaque recherche d’images, sont envoyés à { $service }, qui les facture et les conserve selon ses conditions.
settings-clip-hosted-warning-desktop = Les images quittent cet ordinateur. Une miniature de chaque image de la bibliothèque, et les mots de chaque recherche d’images, sont envoyés à { $service }, qui les facture et les conserve selon ses conditions.
settings-clip-hosted-hint = Rien n’est téléchargé ni exécuté ici : le service apprend chaque image, par lots. Un autre service ou modèle les réapprend toutes, et ce que chacun a appris est conservé, pour pouvoir revenir en arrière.
settings-clip-saved-here = Recherche d’images : s’exécute sur ce serveur
settings-clip-saved-here-desktop = Recherche d’images : s’exécute sur cet ordinateur
settings-clip-saved-hosted = Recherche d’images : s’exécute sur { $service }
settings-clip-hosted-unavailable = Impossible d’utiliser { $service } : { $detail }. Nouvel essai toutes les heures.
settings-clap-heading = Chercher d’après ce à quoi ressemblent les sons
settings-clap-reset = Recherche de sons : retour à la valeur par défaut
settings-clap-saved-on = Recherche de sons : activée
settings-clap-saved-off = Recherche de sons : désactivée
settings-clap-what = Tapez ce que vous cherchez (« tonnerre lointain », « bip d’interface cristallin », « pas sur du gravier ») et les sons qui y ressemblent apparaissent à côté des noms qui contiennent ces mots.
settings-clap-toggle = Activer la recherche de sons
settings-clap-hint = Utilise le modèle CLAP de LAION, exécuté sur le processeur de ce serveur : rien n’en sort. L’activer télécharge le modèle ({ $size } Mo) depuis Hugging Face, puis écoute le début de chaque son en arrière-plan, en basse priorité. Il occupe jusqu’à 1 Go de mémoire environ pendant qu’il travaille ou que quelqu’un cherche. Le désactiver supprime le modèle et garde ce qu’il a appris, pour la prochaine fois.
settings-clap-hint-desktop = Utilise le modèle CLAP de LAION, exécuté sur le processeur de cet ordinateur : rien n’en sort. L’activer télécharge le modèle ({ $size } Mo) depuis Hugging Face, puis écoute le début de chaque son en arrière-plan, en basse priorité. Il occupe jusqu’à 1 Go de mémoire environ pendant qu’il travaille ou que vous cherchez. Le désactiver supprime le modèle et garde ce qu’il a appris, pour la prochaine fois.
settings-clap-ready-empty = Prêt. Les sons sont appris à mesure que des packs sont ajoutés.
settings-clap-ready-all =
    { $count ->
        [one] Prêt : connaît { $count } son
       *[other] Prêt : connaît les { $count } sons
    }
settings-clap-learning = Prêt : apprentissage des sons, { $done } sur { $total } jusqu’ici

## SettingsPage.vue : Étiquetage par IA

settings-ai-service = Service
settings-ai-off = Désactivé
settings-ai-off-hint = Les packs sont étiquetés avec les règles intégrées uniquement.
settings-ai-local = Modèle local
settings-ai-local-hint = Le service llm du fichier compose de Hoard (ou cargo xtask llm). Téléchargez un modèle ci-dessous ; il tourne sur cette machine et rien n’en sort.
settings-ai-chatgpt = Abonnement ChatGPT
settings-ai-chatgpt-hint = Utilise votre abonnement ChatGPT Go, Plus, Pro ou Business au lieu d’une clé d’API.
settings-ai-openrouter-hint = Une seule clé pour des centaines de modèles hébergés, dont certains gratuits. Les modèles qui acceptent les images sont signalés.
settings-ai-openai-hint = Hébergé par OpenAI.
settings-ai-anthropic-hint = Hébergé par Anthropic, via son point d’accès compatible OpenAI.
settings-ai-gemini-hint = Hébergé par Google, via son point d’accès compatible OpenAI.
settings-ai-ollama-hint = Un serveur Ollama que vous faites tourner. Changez l’adresse si ce n’est pas un conteneur nommé ollama.
settings-ai-custom = Autre (compatible OpenAI)
settings-ai-custom-hint = Tout ce qui parle l’API de chat d’OpenAI : LM Studio, vLLM, llama.cpp, LiteLLM…
settings-ai-connecting = Connexion…
settings-ai-connected =
    { $count ->
        [one] Connecté · { $count } modèle
       *[other] Connecté · { $count } modèles
    }
settings-ai-connected-none = Connecté, mais aucun modèle n’est listé ; tapez le nom du modèle
settings-chatgpt-connected-none = Connecté ; tapez le nom du modèle
settings-ai-takes-images = accepte les images
settings-ai-saved = Étiquetage par IA : { $service }
settings-ai-saved-off = L’étiquetage par IA est désactivé
settings-ai-reset = Étiquetage par IA : retour à la valeur par défaut
settings-ai-on = Activé : { $service } · { $model }
settings-ai-url = Adresse
settings-ai-key = Clé d’API
settings-ai-key-saved = Enregistrée (tapez pour la remplacer)
settings-ai-key-paste = Collez votre clé
settings-ai-key-optional = Aucune (facultatif)
settings-ai-key-remove = Retirer
settings-ai-get-key = Obtenir une clé
settings-ai-model = Modèle
settings-ai-model-pick = Choisissez ou tapez un modèle
settings-ai-model-name = Nom du modèle
settings-ai-refresh = Actualiser
settings-ai-connect = Connecter
settings-ai-need-key = Ajoutez la clé d’API pour voir ses modèles
settings-ai-vision = Envoyer aussi les miniatures (nécessite un modèle qui accepte les images)
settings-ai-batch = Par requête
settings-ai-batch-unit = ressources (moins pour les petits modèles locaux, plus pour les gros modèles hébergés)

## SettingsPage.vue : Connexion à ChatGPT

settings-chatgpt-account = Compte
settings-chatgpt-checking = Vérification…
settings-chatgpt-status-as = Connecté en tant que { $email }
settings-chatgpt-status = Connecté
settings-chatgpt-signed-in-as = Connecté à ChatGPT en tant que { $email }
settings-chatgpt-signed-in = Connecté à ChatGPT
settings-chatgpt-signed-out = Déconnecté de ChatGPT
settings-chatgpt-continue = Continuer avec ChatGPT
settings-chatgpt-sign-out = Se déconnecter
settings-chatgpt-sign-out-title = Se déconnecter de ChatGPT ?
settings-chatgpt-sign-out-in-use = L’étiquetage par IA utilise votre abonnement ChatGPT ; il s’arrête jusqu’à ce que vous vous reconnectiez ou choisissiez un autre service.
settings-chatgpt-sign-out-body = Hoard oublie la connexion et demande à OpenAI d’y mettre fin.
settings-chatgpt-manage = Gérer l’utilisation
settings-chatgpt-finish-before = Terminez la connexion dans la fenêtre qui s’est ouverte (
settings-chatgpt-finish-link = la rouvrir
settings-chatgpt-finish-middle = ). Si elle aboutit sur une page qui ne se charge pas, à une adresse commençant par{ " " }
settings-chatgpt-finish-after = , copiez cette adresse et collez-la ici :
settings-chatgpt-paste = L’adresse sur laquelle la connexion a abouti
settings-chatgpt-finish = Terminer
settings-chatgpt-usage = Les requêtes d’étiquetage sont décomptées de l’utilisation de votre abonnement ChatGPT. Gérer l’utilisation montre ce que Hoard a consommé et permet de le limiter. Pour déconnecter Hoard de ChatGPT lui-même, allez dans Paramètres → Sécurité et connexion → Se connecter avec ChatGPT sur chatgpt.com.

## SettingsPage.vue : Modèles locaux

settings-local-downloading-progress = Téléchargement { $percent } % · { $done } sur { $total }
settings-local-downloading = Téléchargement…
settings-local-incomplete = Téléchargement interrompu
settings-local-failed = N’a pas pu démarrer la dernière fois
settings-local-in-use-running = Utilisé · en marche
settings-local-in-use = Utilisé
settings-local-starting = Démarrage…
settings-local-running = En marche
settings-local-ready-images = Prêt · accepte les images
settings-local-ready-text = Prêt · texte uniquement
settings-local-delete-title = Supprimer { $name } ?
settings-local-delete-in-use = L’étiquetage par IA utilise ce modèle ; il s’arrête jusqu’à ce que vous en choisissiez un autre. Ses fichiers sont supprimés du serveur de modèles.
settings-local-delete-downloading = Le téléchargement s’arrête et ce qui a déjà été récupéré est supprimé.
settings-local-delete-body = Ses fichiers sont supprimés du serveur de modèles. Vous pourrez le télécharger à nouveau plus tard.
settings-local-delete = Supprimer
settings-local-deleting = Suppression…
settings-local-cancel = Annuler
settings-local-resume = Reprendre
settings-local-download = Télécharger
settings-local-start-before = Démarrez-le avec{ " " }
settings-local-start-or = { " " }(ou{ " " }
settings-local-start-developing = { " " }en développement), puis{ " " }
settings-local-start-retry = réessayez
settings-local-start-after = . Un serveur qui ne fait tourner qu’un seul modèle convient aussi : indiquez le nom de son modèle ci-dessous.
settings-local-list = Sur le serveur de modèles
settings-local-none = Rien de téléchargé pour l’instant. Choisissez-en un ci-dessous ; il est conservé sur le serveur, et vous pouvez en changer à tout moment.
settings-local-catalog = Modèles à télécharger
settings-local-size = { $size } Go · { $license }
settings-local-other = Un autre modèle de Hugging Face
settings-local-other-placeholder = Un autre de Hugging Face : owner/name-GGUF:Q4_K_M

## SettingsPage.vue : Ajout de packs

settings-store-pages = Pages de boutique
settings-store-pages-what = Les nouveaux packs sont recherchés dans les boutiques en arrière-plan, pour obtenir leur page de boutique, leur vendeur et leur licence.
settings-store-lookup = Rechercher automatiquement les pages de boutique
settings-unreal = Paquets Unreal
settings-unreal-reset = Paquets Unreal : retour à la valeur par défaut
settings-unreal-what = Que faire des fichiers .uasset.
settings-conv = Règles de conversion
settings-conv-what = Convertissez les fichiers que Hoard conserve vers un seul format, par exemple tous les OBJ et FBX en GLB, pour que la bibliothèque soit homogène. Le fichier converti prend la place de l’original, avec son nom, ses étiquettes, ses collections et ses liens. Les fichiers des dossiers surveillés ne sont jamais modifiés.
# A checkbox that turns one conversion rule on or off.
settings-conv-rule-on = Activée
settings-conv-to = Convertir en
# Heading over the file formats a rule converts.
settings-conv-from = Depuis
settings-conv-remove = Retirer
settings-conv-remove-label = Retirer la règle { $n }
settings-conv-taken = Une autre règle convertit déjà ce format
settings-conv-upright = Redresser les modèles lors de la conversion (Y vers le haut, face vers +Z), comme les modèles du pack sont affichés
settings-conv-flac-what = Le son reste exactement le même, dans environ la moitié de la place, et tout le reste du WAV (points de boucle, repères cue, tags) est conservé dans le FLAC : le WAV peut donc être remis exactement tel qu’il était à tout moment. Les WAV avec des échantillons en virgule flottante ou sur 32 bits restent en WAV.
settings-conv-add = Ajouter une règle
# Before a number of days: "Keep originals for [30] days".
settings-conv-keep = Conserver les originaux pendant
settings-conv-keep-days =
    { $count ->
        [one] jour
       *[other] jours
    }
settings-conv-keep-what = D’ici là, la page d’un fichier converti permet de remettre l’original. Les textures ne sont pas copiées dans un modèle converti : il pointe vers les fichiers d’image placés à côté de lui. Les fichiers .mtl ou .bin d’un modèle restent en place.
settings-conv-save = Enregistrer les règles
settings-conv-revert = Annuler les modifications
settings-conv-existing = Convertir les fichiers déjà présents dans la bibliothèque
settings-conv-saved = Règles de conversion enregistrées. Elles s’appliquent aux packs ajoutés à partir de maintenant.
settings-conv-converting =
    { $count ->
        [one] Conversion d’un fichier en arrière-plan
       *[other] Conversion de { $count } fichiers en arrière-plan
    }
settings-conv-nothing = Rien dans la bibliothèque n’a besoin d’être converti.
settings-conv-ask-title =
    { $count ->
        [one] Convertir un fichier déjà présent dans la bibliothèque ?
       *[other] Convertir { $count } fichiers déjà présents dans la bibliothèque ?
    }
settings-conv-ask-body =
    { $packs ->
        [one] Il se trouve dans un pack.
       *[other] Ils se trouvent dans { $packs } packs.
    } Chaque original est conservé { $days } jours, et d’ici là il peut être remis depuis sa page.
settings-conv-ask-busy =
    { $count ->
        [one] Un fichier se trouve dans un pack encore en cours d’ajout ; il sera converti une fois l’ajout terminé.
       *[other] { $count } se trouvent dans des packs encore en cours d’ajout ; ils seront convertis une fois l’ajout terminé.
    }
settings-conv-ask-list = Fichiers qui seraient convertis
settings-conv-ask-more = …et { $count } de plus
settings-conv-ask-skipped = Laissés tels quels :
settings-conv-skip-watched =
    { $count ->
        [one] un fichier dans un dossier surveillé
       *[other] { $count } fichiers dans des dossiers surveillés
    }
settings-conv-skip-unity =
    { $count ->
        [one] un fichier qu’un projet Unity connaît par son nom
       *[other] { $count } fichiers qu’un projet Unity connaît par leur nom
    }
settings-conv-skip-used =
    { $count ->
        [one] un fichier nommé par une scène ou un autre fichier
       *[other] { $count } fichiers nommés par une scène ou un autre fichier
    }
settings-conv-skip-beside =
    { $count ->
        [one] un fichier dont la copie convertie est déjà dans son pack
       *[other] { $count } fichiers dont les copies converties sont déjà dans leurs packs
    }
settings-conv-skip-blender =
    { $count ->
        [one] un fichier que Blender n’a pas encore lu (Paramètres → Outils)
       *[other] { $count } fichiers que Blender n’a pas encore lus (Paramètres → Outils)
    }
settings-conv-ask-new = Enregistrer, nouveaux packs seulement
settings-conv-ask-save-convert =
    { $count ->
        [one] Enregistrer et le convertir
       *[other] Enregistrer et convertir { $count } fichiers
    }
settings-conv-ask-convert =
    { $count ->
        [one] Le convertir
       *[other] Convertir { $count } fichiers
    }
settings-unreal-off = Les laisser tels quels
settings-unreal-off-hint = Convertissez-les depuis la page d’un pack quand vous le souhaitez.
settings-unreal-keep = Convertir, garder les originaux
settings-unreal-keep-hint = Les textures, les sons et les matériaux sortent dans des formats courants ; le .uasset reste aussi.
settings-unreal-replace = Convertir, supprimer les originaux
settings-unreal-replace-hint = Libère de l’espace une fois le contenu extrait. Les maillages attendent toujours le worker Unreal.

## SettingsPage.vue : Téléchargements

settings-tn = Noms des textures
settings-tn-saved = Noms des textures enregistrés
settings-tn-reset = Noms des textures : retour à la valeur par défaut
settings-tn-what = Les cartes d’un matériau sortent nommées d’après lui et d’après le rôle de chaque carte, quel que soit le nom des fichiers : Rock_Normal.png, et non rock_moss_xeokah0_raw_8K_Normal.png. Les fichiers de la bibliothèque ne sont pas modifiés, et une carte nommée par un modèle ou un MTL envoyé avec elle garde ce nom, pour que le modèle la trouve toujours.
settings-tn-toggle = Nommer les cartes de texture d’après leur matériau
settings-tn-style = Style
settings-tn-style-group = Style de nommage
settings-tn-descriptive = Descriptif
settings-tn-short = Court
settings-tn-custom = Personnalisé
settings-tn-pattern = Modèle de nom
settings-tn-hint-name = { " " }le matériau,{ " " }
settings-tn-hint-map = { " " }le rôle de la carte,{ " " }
settings-tn-hint-size = { " " }sa taille quand le jeu existe en plusieurs (ajoutée à la fin si le modèle de nom l’omet).
settings-tn-example = Exemple
settings-tn-suffixes = Suffixes pour chaque type de carte
settings-tn-slots = Dans les téléchargements GLB, une texture intégrée à un modèle sans fichier propre est nommée d’après le matériau et l’emplacement qu’elle occupe, et ces emplacements ont aussi des suffixes :
settings-tn-endings = Terminaisons personnalisées
settings-tn-endings-what = Une texture qui n’est pas l’une des cartes d’un matériau garde son propre nom, sauf s’il se termine par l’une de celles-ci : _diff → _BC envoie rock_diff.png sous le nom rock_BC.png. Laissez la seconde vide pour supprimer la terminaison.
settings-tn-ending-from = Terminaison
settings-tn-ending-from-example = _diff
settings-tn-ending-to = Remplacer par
settings-tn-ending-to-example = _BC
settings-tn-ending-add = Ajouter une terminaison
settings-tn-ending-remove = Retirer
settings-tn-ending-remove-label = Retirer { $ending }

## SettingsPage.vue : Général et Connexion

settings-name = Nom
settings-name-what = Le nom de ce serveur dans la barre du haut, l’onglet du navigateur et sur la page de connexion, pour distinguer vos différents Hoard.
settings-name-what-desktop = Le nom de cette bibliothèque dans la barre du haut.
settings-address = Adresse
settings-address-what = L’adresse que les personnes et les applications utilisent pour joindre Hoard, quand ce n’est pas celle sur laquelle se trouve ce navigateur : celle d’un proxy inverse, d’un tunnel ou un nom de tailnet, comme https://hoard.example.com. Les liens que Hoard fournit (à Claude et aux autres applications, lors de la connexion, sur son certificat) l’utilisent.
settings-public-url = Adresse publique
settings-public-url-placeholder = D’après l’adresse du navigateur (p. ex. https://hoard.example.com)
settings-public-url-default = d’après l’adresse du navigateur

## ProfilerPanel.vue : le profileur en haut de Paramètres → Performances

profiler = Profileur
profiler-what = Ce que Hoard a fait ces dix dernières minutes, mis à jour toutes les deux secondes pendant que vous regardez. Ces données ne sont gardées qu’en mémoire et repartent de zéro quand Hoard redémarre.
profiler-starting = Les premières mesures arrivent dans un instant.
profiler-failed = Impossible de lire le profileur : { $error }
profiler-cpu = Utilisation du processeur
profiler-cpu-hoard = Hoard
profiler-cpu-machine = Machine entière
profiler-cpu-summary = Hoard utilise { $hoard } des processeurs, et la machine entière { $machine }.
profiler-ram = RAM
profiler-ram-peak = Pic
profiler-ram-of = { $used } sur { $total }
profiler-ram-summary = Hoard utilise { $hoard } de RAM, et en a utilisé au plus { $peak } ces dix dernières minutes.
profiler-serving = Requêtes en cours
profiler-working = En cours
profiler-working-none = Rien n’a tourné en arrière-plan.
profiler-working-more =
    { $count ->
        [one] Et encore une.
       *[other] Et encore { $count }.
    }
profiler-requests = Temps passé à servir les requêtes
profiler-requests-what = Chaque barre représente dix secondes de requêtes, leur temps réparti selon l’endroit où il est passé.
profiler-part-db = Base de données
profiler-part-db-title = Attente de la base de données et exécution de ses requêtes SQL
profiler-part-disk = Disque
profiler-part-disk-title = Lecture des fichiers stockés et compression de ce qui est envoyé
profiler-part-send = Envoi
profiler-part-send-title = Attente pendant que le réseau prend ce qui a été envoyé
profiler-part-other = Autre
profiler-part-other-title = Élaboration de la réponse
profiler-request-count =
    { $count ->
        [one] { $count } requête
       *[other] { $count } requêtes
    }
profiler-requests-none = Aucune requête ces dix dernières minutes.
profiler-slowest = La plus lente : { $route }, { $time }
profiler-routes = Par route
profiler-route = Route
profiler-count = Requêtes
profiler-average = Moyenne
profiler-p95 = 5 % les plus lentes
profiler-split = Où le temps est passé
profiler-sent = Envoyé
profiler-portal = les fichiers de l’interface elle-même

## SettingsPage.vue : Performances

settings-render = Miniatures 3D
settings-render-toggle = Créer les miniatures des modèles 3D
settings-gpu-on = Sur le GPU : { $renderer }
settings-gpu-software = Aucun GPU trouvé, donc en logiciel ({ $renderer })
settings-gpu-none = Aucun GPU ni moteur de rendu logiciel trouvé, donc les modèles n’ont pas de miniature
settings-thumbs = Miniatures
settings-thumbs-what = Pour les nouvelles miniatures et vues de modèles. Les transparentes et le pixel art restent en PNG. Réduire les miniatures dans Maintenance refait celles que vous avez.
settings-previews-size = Aperçus : { $size }
settings-thumbs-saved = Miniatures : { $quality }
settings-thumbs-format = Format
settings-thumbs-quality = Qualité { $quality }.
settings-thumbs-small = Plus légères
settings-thumbs-small-hint = Bien dans la grille ; un peu flou de près.
settings-thumbs-balanced = Équilibrées
settings-thumbs-balanced-hint = Difficile à distinguer de l’original à la taille d’une miniature.
settings-thumbs-sharp = Plus nettes
settings-thumbs-sharp-hint = Environ deux fois plus lourd qu’Équilibrées.
settings-thumbs-lossless = Sans perte
settings-thumbs-lossless-hint = PNG, exactement telles que créées. Plusieurs fois plus lourd. Jamais envoyées en AVIF.
settings-thumbs-avif-what = L’AVIF fait souvent moitié moins lourd. Chaque miniature est convertie la première fois qu’elle est affichée.
settings-thumbs-avif-now = Tout convertir maintenant
settings-thumbs-avif-converting = Conversion…
settings-motion-thumbs = Miniatures animées pour les animations
settings-motion-thumbs-what = Chaque animation reçoit aussi un court film en boucle, joué quand vous pointez sa vignette. Ils pèsent environ 40 Ko chacun et prennent un moment à créer pour une grande bibliothèque. Désactiver l'option les supprime.
settings-limits = Travail et limites
settings-limits-what = Chaque valeur s’enregistre quand vous quittez le champ ou appuyez sur Entrée. Les tailles prennent une unité : 100 GB, 512 MB, 1,5 TB ; un nombre seul compte en GB.
settings-workers = Workers en arrière-plan
settings-workers-unit = tâches à la fois (ingestion, étiquetage, conversions)
settings-max-upload = Envoi le plus gros
settings-max-upload-unit = par fichier
settings-max-expand = Pack le plus gros
settings-max-expand-unit = une fois décompressé
settings-conversion-cache = Téléchargements convertis
settings-conversion-cache-unit-now = conservés au maximum, { $size } actuellement ; les moins récemment utilisés partent en premier
settings-conversion-cache-unit = conservés au maximum ; les moins récemment utilisés partent en premier

## SettingsPage.vue : Maintenance

settings-updates = Mises à jour
settings-version = Ceci est Hoard { $version }.
settings-update-check = Vérifier sur GitHub toutes les quelques heures si une nouvelle version est sortie
settings-desktop-updates = Hoard cherche une nouvelle version à chaque démarrage, la télécharge en arrière-plan et l’installe quand vous le quittez.
settings-backups = Sauvegardes planifiées
settings-backups-what = La base de données se sauvegarde toute seule ; chaque sauvegarde apparaît dans l’historique ci-dessous.
settings-backup-every = Sauvegarder
settings-backup-off = Jamais
settings-backup-daily = Chaque jour
settings-backup-weekly = Chaque semaine
settings-backup-keep = Garder
settings-backup-keep-unit = sauvegardes planifiées les plus récentes
settings-backup-copy-to = Copier aussi dans
settings-backup-copy-to-none = Aucun autre dossier
settings-backups-kept = Chacune va dans le dossier des sauvegardes sous le nom hoard-auto-…db, et dans l’autre dossier (un autre disque, ou un dossier qu’une application de synchronisation sauvegarde) s’il y en a un, indiqué par son chemin complet sur le serveur. Seules les sauvegardes planifiées plus anciennes sont supprimées ; celles faites à la main restent.
about-what = Hoard range les ressources de jeu (modèles, textures, sons, musiques, sprites et plus) dans une bibliothèque où vous pouvez chercher, prévisualiser et vous servir, partagée avec toutes les personnes à qui vous donnez accès.
about-version = Version
about-edition = Édition
about-edition-server = Serveur
about-edition-desktop = Application de bureau
about-edition-steam = Application de bureau Steam
about-made-by = Créé par
about-source = Code source sur GitHub
about-issues = Signaler un problème
about-discord = Discuter avec nous sur Discord
about-steam = Hoard sur Steam
about-license = Licence
about-license-what = Hoard est open source sous la licence MIT No Attribution : utilisez-le, modifiez-le et partagez-le comme bon vous semble.
about-downloaded = Téléchargé au besoin
about-downloaded-what = Hoard n’est pas livré avec ces outils. Il télécharge chacun d’eux la première fois que quelque chose en a besoin, et chacun est soumis à sa propre licence.
about-blender = Convertit les modèles 3D et crée leurs miniatures.
about-ffmpeg = Convertit le son et la vidéo et crée leurs aperçus.
about-siglip = Trouve des images d’après ce qu’elles montrent.
about-clap = Trouve des sons d’après ce à quoi ils ressemblent.
about-models = Les modèles que vous choisissez pour le serveur d’IA local sont soumis aux licences de leurs auteurs.
about-built-with = Conçu avec
about-built-with-what = Les bibliothèques open source dont Hoard est fait, avec leurs licences.
about-steamworks = La version Steam inclut aussi le SDK Steamworks, © Valve Corporation.
about-licenses-loading = Chargement de la liste…
about-licenses-failed = Impossible de charger la liste.
about-filter = Filtrer par nom ou licence
about-count = { $shown } sur { $total }
about-no-license = Aucune licence indiquée
about-report = Signaler un problème…
report-title = Signaler un problème
report-what = Voici ce dont un ticket sur GitHub a besoin pour que le problème soit examiné. Rien de privé n’y figure : les jetons d’accès, mots de passe, clés, adresses e-mail et votre nom dans les chemins de dossiers sont retirés. Relisez-le avant de l’envoyer.
report-loading = Préparation du rapport…
report-failed = Le rapport n’a pas pu être préparé : { $error }
report-log = Ce que le serveur a journalisé dernièrement
report-copy = Copier
report-copied = Rapport copié.
report-github = Ouvrir un ticket sur GitHub
report-github-hint = Le nouveau ticket est rempli avec ceci. Si le journal est trop long, seules ses dernières lignes y vont ; Copier contient tout.
report-close = Fermer

## Types de ressources (api.ts)

api-kind-texture = Textures
api-kind-vector = Art vectoriel
api-kind-sheet = Planches de sprites
api-kind-audio = Audio
api-kind-model = Modèles
api-kind-animation = Animations
api-kind-font = Polices
api-kind-shader = Shaders
api-kind-material = Matériaux
api-kind-video = Vidéo
api-kind-document = Documents
api-kind-data = Données
api-kind-other = Autres
api-kind-texture-name = textures
api-kind-vector-name = art vectoriel
api-kind-sheet-name = planches de sprites
api-kind-audio-name = fichiers audio
api-kind-model-name = modèles
api-kind-animation-name = animations
api-kind-font-name = polices
api-kind-shader-name = shaders
api-kind-material-name = matériaux
api-kind-video-name = vidéos
api-kind-document-name = documents
api-kind-data-name = données
api-kind-other-name = autres

## Notifications, boîtes de dialogue et chargement (ui.ts, UiLayer.vue, lazy.ts)

ui-undo = Annuler
ui-today = aujourd’hui
ui-yesterday = hier
ui-days-ago =
    { $days ->
        [one] il y a { $days } jour
       *[other] il y a { $days } jours
    }
ui-notifications = Notifications
ui-dismiss = Ignorer
ui-cancel = Annuler
ui-lazy-loading = Chargement en cours : { $what }…
ui-lazy-failed = Impossible de charger { $what }.
ui-lazy-reload = Recharger la page

## ResetButton.vue

reset-title = Rétablir la valeur par défaut : { $value }
reset-label = Rétablir la valeur par défaut, { $value }

## SizeInput.vue

size-invalid = Une taille comme 100 GB, 512 MB ou 1,5 TB
size-hint = Une taille comme 100 GB, 512 MB ou 1,5 TB ; un nombre seul compte en GB

## AuthPanel.vue (Paramètres → Authentification)

auth-load-failed = Impossible de charger les paramètres d’authentification : { $error }
auth-saved = Enregistré. Cela s’applique à la prochaine connexion.
auth-method-internal = Interne
auth-method-internal-hint = Des noms et mots de passe gérés par Hoard, ajoutés dans Paramètres → Compte.
auth-method-ldap-hint = Un annuaire vérifie le nom et le mot de passe saisis sur la page de connexion : OpenLDAP, FreeIPA, LLDAP ou un autre serveur LDAP.
auth-method-ad-hint = Les comptes du domaine Windows se connectent avec leurs nom et mot de passe habituels.
auth-method-oauth-hint = Un bouton « Se connecter avec… » qui passe par Google, Microsoft, GitHub, Keycloak, Authentik ou un autre fournisseur, puis revient.
auth-session-unused = Après 30 jours sans utilisation
auth-session-1h = 1 heure après la connexion
auth-session-8h = 8 heures après la connexion
auth-session-day = Un jour après la connexion
auth-session-week = Une semaine après la connexion
auth-session-30d = 30 jours après la connexion
auth-session-hours =
    { $hours ->
        [one] { $hours } heure après la connexion
       *[other] { $hours } heures après la connexion
    }
auth-role-view = Peut consulter
auth-role-edit = Peut modifier
auth-role-admin = Admin
auth-role-word-view = peut consulter
auth-role-word-edit = peut modifier
auth-role-word-admin = admin
auth-role-word-default = { $role } (le rôle par défaut)
auth-tls-starttls-hint = LDAP simple (port 389), passé en TLS avant tout envoi.
auth-tls-ldaps-hint = TLS dès le départ (port 636).
auth-tls-none = Aucune
auth-tls-none-hint = Les mots de passe traversent le réseau en clair. Seulement sur un réseau de confiance.
auth-test-failed = Impossible de tester : { $error }
auth-preset-custom = Autre (OpenID Connect)
auth-preset-custom-note = Keycloak, Authentik, Authelia, Okta, Zitadel et d’autres : indiquez l’émetteur et Découvrir remplit les adresses.
auth-preset-google-note = Créez un client OAuth (Application Web) dans Google Cloud Console → API et services → Identifiants. Google n’envoie aucun groupe : attribuez les rôles dans Paramètres → Compte, et désactivez la création de comptes à la première connexion, sauf si toute personne ayant un compte Google doit pouvoir entrer.
auth-preset-microsoft-note = Inscrivez une application dans Entra ID → Inscriptions d’applications (une redirection Web vers l’adresse ci-dessous). Remplacez « organizations » par l’ID de votre locataire pour n’admettre que votre organisation. Pour des rôles issus des groupes, ajoutez la revendication groups dans Configuration du jeton : elle liste les ID des groupes.
auth-preset-github-note = Créez une OAuth app dans GitHub → Settings → Developer settings. GitHub n’envoie aucun groupe : attribuez les rôles dans Paramètres → Compte, et désactivez la création de comptes à la première connexion, sauf si toute personne ayant un compte GitHub doit pouvoir entrer.
auth-preset-gitlab-note = Créez une application dans GitLab → Preferences → Applications (ou dans la zone d’administration de votre instance : changez gitlab.com dans les adresses).
auth-discovered = Adresses du fournisseur remplies.
auth-copied = Copié
auth-copy-failed = Impossible de copier : sélectionnez le texte et copiez-le vous-même
auth-sign-in-with = Se connecter avec
auth-own-accounts-still-work = Les comptes propres à Hoard se connectent toujours avec leur propre mot de passe : un annuaire ou un fournisseur en panne ou mal configuré ne peut pas vous bloquer dehors.
auth-sessions = Sessions
auth-sign-out-after = Déconnecter les personnes
auth-ldap-server = Serveur LDAP
auth-server-type = Type de serveur
auth-server-type-custom = Personnalisé
auth-servers = Serveurs
auth-server-n = Serveur { $n }
auth-remove-server-n = Retirer le serveur { $n }
auth-remove = Retirer
auth-add-server = Ajouter un serveur
auth-servers-hint = Le suivant est essayé quand l’un est injoignable.
auth-security = Sécurité
auth-connection-security = Sécurité de la connexion
auth-skip-verify = Ne pas vérifier le certificat du serveur (seulement pour faire des essais)
auth-ca = Autorité de certification
auth-ca-placeholder = -----BEGIN CERTIFICATE-----  (vide : celles auxquelles le système fait confiance)
auth-lookup = Recherche des personnes
auth-anonymously = Anonymement
auth-reader-account = Compte de lecture
auth-reader-password = Son mot de passe
auth-saved-type-to-replace = Enregistré : tapez pour remplacer
auth-people-under = Les personnes sont sous
auth-name-attribute = Attribut du nom
auth-only-matching = Seulement les entrées correspondant à
auth-groups-under = Les groupes sont sous
auth-groups-under-placeholder = ou=groups,dc=example,dc=com  (vide : memberOf uniquement)
auth-their-groups = Leurs groupes
auth-group-filter-hint = { "{" }dn{ "}" } est l’entrée de la personne, { "{" }user{ "}" } le nom avec lequel elle s’est connectée.
auth-test = Tester
auth-test-hint = Joint chaque serveur et se connecte avec le compte de lecture. Avec un nom et un mot de passe, se connecte aussi en tant que cette personne et montre ce que Hoard en déduirait. Rien n’est enregistré.
auth-test-user = Une personne à connecter
auth-test-user-placeholder = Une personne à connecter (facultatif)
auth-test-password = Son mot de passe
auth-testing = Test en cours…
auth-test-connectivity = Tester la connectivité
auth-test-server-ok = joint, et le compte de lecture s’est connecté
auth-test-signed-in-before = { "" }
auth-test-signed-in-middle = { " " }s’est connecté (
auth-test-signed-in-after = ).
auth-test-role = Son rôle serait : { $role }.
auth-test-no-role = Cette personne n’est dans aucun des groupes listés sous Rôles : elle serait refusée.
auth-test-groups = Groupes : { $groups }
auth-test-no-groups = Aucun groupe trouvé.
auth-provider = Fournisseur
auth-redirect = Adresse de redirection
auth-copy = Copier
auth-redirect-hint = Donnez-la au fournisseur quand vous créez le client de Hoard. Elle vient de l’adresse publique définie dans Paramètres → Connexion, ou de l’adresse que vous utilisez.
auth-issuer = Émetteur
auth-discover = Découvrir
auth-client-id = ID client
auth-client-secret = Secret client
auth-authorization-url = Adresse d’autorisation
auth-token-url = Adresse du jeton
auth-userinfo-url = Adresse des informations utilisateur
auth-userinfo-placeholder = (vide : le jeton d’ID indique de qui il s’agit)
auth-scopes = Portées
auth-name-from = Nom tiré de
auth-groups-from = Groupes tirés de
auth-groups-from-placeholder = (aucun)
auth-button = Bouton
auth-button-placeholder = votre compte
auth-accounts = Comptes
auth-provision = Créer un compte Hoard à la première connexion d’une personne
auth-provision-on-hint = Sinon, un administrateur ajoute d’abord chaque personne, dans Paramètres → Compte.
auth-provision-off-hint = Un administrateur ajoute d’abord chaque personne, dans Paramètres → Compte, en choisissant comment elle se connecte.
auth-open-to-anyone = Toute personne ayant un compte { $provider } peut entrer. Ajoutez des groupes ci-dessous sous Peut consulter, ou désactivez ceci et ajoutez les personnes vous-même.
auth-new-accounts = Nouveaux comptes
auth-roles-from-groups = Rôles d’après les groupes
auth-roles-hint = Les groupes dont les membres obtiennent chaque rôle, un par ligne, par nom ou par DN. Le plus élevé qui correspond l’emporte, et c’est vérifié à chaque connexion. Si aucun n’est listé, les nouveaux comptes reçoivent le rôle ci-dessus et un administrateur le change ensuite. Si des groupes sont listés sous Peut consulter, seuls les membres d’un groupe listé peuvent se connecter.
auth-roles-hint-oauth = Les groupes dont les membres obtiennent chaque rôle, un par ligne, par nom ou par DN (ou ce que liste la revendication des groupes). Le plus élevé qui correspond l’emporte, et c’est vérifié à chaque connexion. Si aucun n’est listé, les nouveaux comptes reçoivent le rôle ci-dessus et un administrateur le change ensuite. Si des groupes sont listés sous Peut consulter, seuls les membres d’un groupe listé peuvent se connecter.
auth-saving = Enregistrement…
auth-save = Enregistrer

## HttpsPanel.vue (Paramètres → Connexion)

https-load-failed = Impossible de charger HTTPS : { $error }
https-off-redirecting = HTTPS est désactivé. Redirection vers http://
https-now-on = HTTPS est activé
https-saved = Enregistré. Les nouvelles connexions utilisent le nouveau certificat.
https-now-off = HTTPS est désactivé
https-http3-failed-toast = HTTP/3 n’a pas pu démarrer : { $error }
https-copied = Copié
https-copy-failed = Impossible de copier : sélectionnez le texte et copiez-le vous-même
https-mode-off = Désactivé
https-mode-off-hint = http:// simple uniquement.
https-mode-own = Le certificat de Hoard
https-mode-own-hint = Hoard en crée un pour les adresses de ce serveur. Installez une fois l’autorité de certification de Hoard sur chaque appareil, et ses navigateurs lui font confiance.
https-mode-custom = Mon certificat
https-mode-custom-hint = Un certificat que vous avez déjà, p. ex. obtenu avec tailscale cert ou Let’s Encrypt. Les navigateurs lui font confiance sans rien installer.
https-untrusted-own = Ce navigateur ne lui fait pas encore confiance pour { $host } : installez l’autorité de certification de Hoard ci-dessous. Vous verrez un avertissement que vous pourrez passer.
https-untrusted-uncovered = Ce navigateur ne lui fait pas encore confiance pour { $host } : le certificat ne couvre pas cette adresse. Vous verrez un avertissement que vous pourrez passer.
https-untrusted = Ce navigateur ne lui fait pas encore confiance pour { $host }. Vous verrez un avertissement que vous pourrez passer.
https-now = Actuellement
https-start-failed = HTTPS est configuré mais n’a pas pu démarrer : { $error }
https-on-port = HTTPS est activé, sur le port { $port } comme http://.
https-using-it = Vous l’utilisez en ce moment.
https-open = Ouvrir { $url }
https-trusted = Ce navigateur lui fait confiance.
https-cert-for-before = Certificat pour{ " " }
https-cert-for-after = , jusqu’au { $date }.
https-cert-for-after-own = , jusqu’au { $date } (renouvelé automatiquement).
https-not-covered-before = Il ne couvre pas{ " " }
https-not-covered-after = , l’adresse que vous utilisez.
https-is-off = HTTPS est désactivé : les pages sont servies en http:// simple.
https-addresses = Adresses
https-addresses-hint = Le certificat couvre les adresses que Hoard a trouvées, et celles que vous ajoutez (un nom donné par votre routeur, un nom Tailscale).
https-remove-name = Retirer { $name }
https-another-address = Autre adresse
https-another-address-placeholder = Autre adresse, p. ex. nas.local
https-add = Ajouter
https-your-cert = Votre certificat
https-saved-cert-before = Enregistré :{ " " }
https-saved-cert-after = { " " }émis par { $issuer }, jusqu’au { $date }. Collez-en un nouveau pour le remplacer.
https-pem-before = Le certificat (avec sa chaîne) et sa clé privée, en texte PEM : les fichiers qui commencent par{ " " }
https-pem-after = .
https-certificate = Certificat
https-choose-file = Choisir un fichier…
https-private-key = Clé privée
https-checked-before = Pour{ " " }
https-checked-after = , émis par { $issuer }, jusqu’au { $date }.
https-checked-after-days =
    , émis par { $issuer }, jusqu’au { $date } ({ $days ->
        [one] { $days } jour
       *[other] { $days } jours
    }).
https-self-signed = Il est autosigné : les navigateurs afficheront un avertissement.
https-key-matches = La clé correspond.
https-where-to-get = Où en obtenir un
https-get-tailscale-before = activez HTTPS pour votre tailnet, puis exécutez{ " " }
https-get-tailscale-after = { " " }sur le serveur et envoyez les deux fichiers. Ils durent 90 jours : relancez la commande et renvoyez-les avant leur expiration.
https-get-acme-title = Let’s Encrypt ou un autre client ACME
https-get-acme-before = (certbot, acme.sh, lego) pour un domaine qui vous appartient : envoyez{ " " }
https-get-acme-and = { " " }et{ " " }
https-get-acme-after = .
https-get-proxy-title = Un proxy inverse
https-get-proxy = (Caddy, Traefik, nginx, Cloudflare Tunnel) qui détient le certificat : laissez HTTPS désactivé ici et laissez le proxy transmettre en http:// simple à Hoard.
https-trying = Essai en cours…
https-apply = Appliquer
https-apply-hint = Hoard essaie d’abord la nouvelle configuration, et garde l’actuelle si elle ne fonctionne pas.
https-trust-title = Faire confiance à Hoard sur vos appareils
https-download-ca = Télécharger hoard-ca.crt
https-trust-hint = Installez une fois l’autorité de certification de Hoard sur chaque appareil, et ses navigateurs font confiance à ce serveur (et à tout certificat que Hoard créera pour lui plus tard). Elle ne peut se porter garante que de ce Hoard. Empreinte (SHA-256) :
https-ca-windows = Ouvrez le fichier → Installer le certificat → Utilisateur actuel → Placer tous les certificats dans le magasin suivant → Parcourir → Autorités de certification racines de confiance → Terminer. Redémarrez le navigateur.
https-ca-macos = Ouvrez le fichier ; il s’ajoute à Trousseaux d’accès. Double-cliquez sur « { $subject } » → Se fier → Lors de l’utilisation de ce certificat : Toujours approuver.
https-ca-ios-title = iPhone et iPad
https-ca-ios = Téléchargez-le dans Safari et autorisez-le. Réglages → Profil téléchargé → Installer. Puis Réglages → Général → Informations → Réglages de confiance des certificats → activez « { $subject } ».
https-ca-android = Paramètres → Sécurité → Autres paramètres de sécurité → Chiffrement et identifiants → Installer un certificat → Certificat CA, puis choisissez le fichier. (Les noms varient un peu d’un téléphone à l’autre ; cherchez « certificat » dans les Paramètres.)
https-ca-linux-1 = Debian et Ubuntu :{ " " }
https-ca-linux-2 = . Fedora et Arch :{ " " }
https-ca-linux-3 = . Chrome sous Linux garde sa propre liste : Paramètres → Confidentialité et sécurité → Sécurité → Gérer les certificats → Importer.
https-ca-firefox = Firefox garde sa propre liste : Paramètres → Vie privée et sécurité → Certificats → Afficher les certificats → Autorités → Importer, et cochez « Confirmer cette AC pour identifier des sites web ».
https-ca-cli-title = L’outil en ligne de commande hoard
https-ca-cli-before = { "" }
https-ca-cli-middle = , ou indiquez le chemin dans{ " " }
https-ca-cli-after = .
https-options = Options
https-redirect = Rediriger les navigateurs des autres appareils de http:// vers https://
https-redirect-hint = Seuls les chargements de pages sont redirigés. L’outil en ligne de commande hoard, le worker Unreal, les scripts et les applications d’IA continuent de fonctionner en http://, tout comme http://localhost sur le serveur lui-même.
https-http3-hint-before = Plus rapide sur un Wi-Fi capricieux et sur les téléphones. Nécessite HTTPS, et le port UDP { $port } ouvert en plus du TCP (dans Docker :{ " " }
https-http3-hint-after = ).
https-http3-running = En marche.
https-http3-waits = Il démarre quand HTTPS est activé.
https-http3-failed = HTTP/3 n’a pas pu démarrer : { $error }
https-lockout-title = Si vous êtes bloqué dehors
https-lockout-1 = { "" }
https-lockout-2 = { " " }sur le serveur lui-même fonctionne toujours en http:// simple. Ou désactivez HTTPS depuis la ligne de commande du serveur et redémarrez Hoard :{ " " }
https-lockout-3 = { " " }(dans Docker :{ " " }
https-lockout-4 = , puis{ " " }
https-lockout-5 = ).

## AccountPanel.vue (Paramètres → Compte)

account-role-view = Peut consulter
account-role-view-hint = Chercher, prévisualiser et télécharger
account-role-edit = Peut modifier
account-role-edit-hint = Aussi ajouter, modifier, étiqueter, regrouper en collections et archiver
account-role-admin = Admin
account-role-admin-hint = Aussi les Paramètres, les comptes et la maintenance, et la suppression définitive
account-source-internal = Un mot de passe Hoard
account-source-ldap = Son mot de passe { $directory }
account-source-oauth = Connexion avec { $provider }
account-source-word-directory = annuaire
account-load-failed = Impossible de charger les comptes : { $error }
account-failed = { $what } : { $error }
account-password-failed = Impossible de changer votre mot de passe
account-password-changed = Mot de passe changé. Vos autres appareils sont déconnectés.
account-add-failed = Impossible d’ajouter { $name }
account-added = { $name } peut maintenant se connecter.
account-role-failed = Impossible de changer ce que { $name } peut faire
account-remove-title = Retirer { $name } ?
account-remove-body = Cette personne est déconnectée partout et ne peut plus se connecter. Rien de ce qu’elle a ajouté à la bibliothèque n’est retiré.
account-remove = Retirer
account-remove-failed = Impossible de retirer { $name }
account-token-failed = Impossible de créer le jeton
account-copied = Copié
account-copy-token-failed = Impossible de copier : sélectionnez le jeton et copiez-le vous-même
account-copy-failed = Impossible de copier : sélectionnez le texte et copiez-le vous-même
account-revoke-title = Révoquer « { $name } » ?
account-revoke-body = Tout ce qui l’utilise encore est refusé à partir de maintenant.
account-revoke = Révoquer
account-revoke-failed = Impossible de révoquer « { $name } »
account-disconnect-title = Déconnecter { $app } ?
account-disconnect-body = L’application est refusée à partir de maintenant, et doit être connectée à nouveau pour utiliser Hoard.
account-disconnect = Déconnecter
account-disconnect-failed = Impossible de déconnecter { $app }
account-guest-failed = Impossible de modifier ce réglage
account-never = jamais
account-you = Vous
account-sign-out = Se déconnecter
account-signed-in-as-before = Connecté en tant que{ " " }
account-signed-in-as-admin = , qui est administrateur.
account-signed-in-as-edit = , qui peut modifier la bibliothèque.
account-signed-in-as-view = , qui peut consulter mais rien modifier.
account-app-signin = L’application Hoard vous connecte elle-même, il n’y a donc pas de mot de passe. Créez ci-dessous un jeton d’accès pour les extensions des moteurs et la commande hoard.
account-password-at-provider = Votre mot de passe est géré par le fournisseur avec lequel vous vous connectez : c’est là qu’il se change.
account-password-at-directory = Votre mot de passe est géré par l’annuaire avec lequel vous vous connectez : c’est là qu’il se change.
account-current-password = Mot de passe actuel
account-new-password = Nouveau mot de passe
account-new-password-again = Nouveau mot de passe, à nouveau
account-change-password = Changer le mot de passe
account-password-mismatch = Les nouveaux mots de passe ne correspondent pas.
account-password-rule = Au moins 8 caractères.
account-people = Personnes
account-people-hint = Chaque personne peut consulter (chercher, prévisualiser et télécharger), modifier aussi la bibliothèque (ajouter, modifier, étiqueter, regrouper en collections et archiver), ou être en plus administrateur (Paramètres, comptes et maintenance, et suppression définitive).
account-you-tag = (vous)
account-signs-in-oauth = Se connecte avec le fournisseur OAuth
account-signs-in-directory = Se connecte avec l’annuaire
account-since = depuis le { $date }
account-what-can-do = Ce que { $name } peut faire
account-name = Nom
account-how-sign-in = Comment cette personne se connecte
account-password-label = Mot de passe (au moins 8 caractères)
account-password-placeholder = Mot de passe (8 caractères ou plus)
account-what-they-can-do = Ce qu’elle peut faire
account-add = Ajouter
account-ai-apps = Claude et autres applications d’IA
account-ai-apps-hint = Connectez Claude (ou toute application MCP) et il peut chercher et récupérer des ressources pour vous, et décrire et étiqueter ce que Hoard n’a pas encore traité. Il se connecte à Hoard la première fois ; vous l’autorisez ici, dans le navigateur.
account-mcp-address = Adresse MCP de Hoard
account-copy = Copier
account-claude-code-1 = exécutez{ " " }
account-claude-code-2 = , puis{ " " }
account-claude-code-3 = { " " }pour vous connecter.{ " " }
account-claude-code-4 = { " " }décrit un pack.
account-claude-ai = Paramètres → Connecteurs → Ajouter un connecteur personnalisé, collez l’adresse, puis Connecter. Ces applications se connectent depuis les serveurs d’Anthropic : Hoard doit donc être joignable depuis Internet en HTTPS.
account-claude-ai-local = Paramètres → Connecteurs → Ajouter un connecteur personnalisé, collez l’adresse, puis Connecter. Ces applications se connectent depuis les serveurs d’Anthropic : Hoard doit donc être joignable depuis Internet en HTTPS (ce n’est pas le cas de cette adresse : définissez l’adresse publique dans Paramètres → Connexion).
account-connection-meta = pour { $user } depuis { $since } · dernière utilisation { $used }
account-no-apps = Aucune application connectée pour l’instant.
account-live = Utilisent Hoard en ce moment
account-live-hint = Les applications qui ont demandé quelque chose à Hoard ces 10 dernières minutes. La liste se met à jour toute seule.
account-live-none = Rien n’a rien demandé à Hoard ces 10 dernières minutes.
account-live-last = dernier : { $what } · { $when } · requêtes : { $calls }
account-tokens = Jetons d’accès
account-tokens-hint-1 = Pour l’outil en ligne de commande{ " " }
account-tokens-hint-2 = , les scripts, les clients MCP et le worker Unreal : ils l’envoient sous la forme{ " " }
account-tokens-hint-3 = . Un jeton peut faire ce que vous choisissez, dans la limite de vos propres droits, mais jamais gérer les comptes ni les jetons. Un jeton qui peut seulement consulter suffit pour chercher, télécharger et synchroniser un projet.
account-tokens-admin = Vous voyez les jetons et applications connectées de tout le monde ; les autres voient les leurs.
account-made-before = { "" }
account-made-after = { " " }: copiez-le maintenant. Il ne sera plus affiché.
account-new-token = Le nouveau jeton
account-cli-1 = Pour la CLI :{ " " }
account-cli-2 = , ou mettez-le dans{ " " }
account-cli-3 = .
account-done = Terminé
account-token-meta = créé { $made } · dernière utilisation { $used }
account-token-meta-by = créé { $made } par { $by } · dernière utilisation { $used }
account-no-tokens = Aucun jeton pour l’instant.
account-token-name = Ce qui l’utilisera
account-token-name-placeholder = Ce qui l’utilisera, p. ex. « Worker Unreal »
account-token-role = Ce qu’il peut faire
account-make-token = Créer un jeton
account-guests = Invités
account-guests-allow = Permettre à toute personne qui peut joindre le serveur de naviguer sans se connecter
account-guests-hint = Les invités peuvent chercher, prévisualiser et télécharger, mais pas ajouter, modifier ou supprimer quoi que ce soit, ni voir les Paramètres.

## SignIn.vue

signin-create-title = Créez votre compte
signin-title = Connexion
signin-setup-lede = Ce serveur n’a pas encore de compte. Celui que vous créez ici peut tout faire ; vous pourrez ajouter des comptes pour d’autres personnes plus tard depuis Paramètres → Compte.
signin-overlay-lede = Ici, la navigation est ouverte à tous, mais les modifications nécessitent un compte.
signin-with = Se connecter avec { $provider }
signin-or-password = ou avec un nom et un mot de passe
signin-username = Nom d’utilisateur
signin-name = Nom
signin-new-password = Nouveau mot de passe
signin-password = Mot de passe
signin-password-rule = Au moins 8 caractères.
signin-confirm-password = Confirmez le mot de passe
signin-mismatch = Les deux mots de passe ne correspondent pas.
signin-setup-code = Code de configuration
signin-setup-code-hint-before = Le code donné à ce serveur dans{ " " }
signin-setup-code-hint-after = .
signin-creating = Création…
signin-signing-in = Connexion…
signin-create = Créer le compte
signin-submit = Se connecter
signin-keep-browsing = Continuer à naviguer
signin-directory-hint = Utilisez vos nom et mot de passe { $directory }.
signin-forgot-1 = Mot de passe Hoard oublié ? Sur le serveur, exécutez{ " " }
signin-forgot-2 = { " " }(dans Docker,{ " " }
signin-forgot-3 = ) pour en obtenir un nouveau.

## SetupWizard.vue (première visite sur un nouveau serveur)

setup-title = Où ranger votre bibliothèque ?
setup-lede = Choisissez un dossier du serveur où Hoard rangera vos packs, ou indiquez-lui un dossier de packs que vous avez déjà. Vous pourrez ajouter d’autres emplacements, sur d’autres disques, plus tard depuis les Paramètres.
setup-lede-desktop = Choisissez un dossier où Hoard rangera vos packs, ou indiquez-lui un dossier de packs que vous avez déjà. Vous pourrez ajouter d’autres emplacements, sur d’autres disques, plus tard depuis les Paramètres.
setup-unreachable = Impossible de joindre le serveur : { $error }
setup-unreachable-desktop = Hoard ne répond pas : { $error }
setup-try-again = Réessayer
setup-loading = Chargement…
setup-start = Créer la bibliothèque
setup-ready = Votre bibliothèque est prête
setup-managed-before = Les nouveaux packs vont dans{ " " }
setup-managed-after = . Déposez des fichiers, des dossiers ou des archives n’importe où sur la page pour ajouter les premiers.
setup-watched-before = Hoard surveille{ " " }
setup-watched-after = { " " }; les packs apparaissent à mesure qu’il les trouve. Pour pouvoir aussi envoyer des fichiers, ajoutez une bibliothèque gérée par Hoard depuis Paramètres → Emplacements de la bibliothèque.
setup-open = Ouvrir la bibliothèque

## ConnectApp.vue (une application comme Claude demande à se connecter)

connect-title = Connecter { $app } ?
connect-lede-1 = { " " }pourra chercher, télécharger, décrire et étiqueter tout le contenu de ce Hoard, en agissant en tant que{ " " }
connect-lede-2 = . Elle ne peut pas gérer les comptes ni les jetons d’accès.
connect-back-before = Vous reviendrez sur{ " " }
connect-back-after = . Déconnectez-la à tout moment dans Paramètres → Compte.
connect-allow = Autoriser
connect-cancel = Annuler
connect-failed = Impossible de connecter cette application
connect-go-library = Aller à la bibliothèque
connect-checking = Vérification…

## InsecureNotice.vue (la ligne affichée en http:// simple)

insecure-text = Cette page est servie en http:// simple, alors que certaines fonctions nécessitent HTTPS : la 3D avec WebGPU, le presse-papiers moderne, l’installation de Hoard comme application et les connecteurs Claude.
insecure-switch = Passer en https://
insecure-set-up = Configurer HTTPS
insecure-dismiss = Ne plus afficher

## FormatWizard.vue (Paramètres → Fichiers des boutiques → apprendre à Hoard le fichier d'une boutique)

formatwizard-too-big = Ce fichier est trop gros pour être lu comme fichier de boutique.
formatwizard-not-json = Ce fichier n’est pas du JSON valide, il est donc affiché comme texte.
formatwizard-read-failed = Impossible de lire le fichier : { $error }
formatwizard-back-title = Revenir au formulaire ?
formatwizard-back-body = Les modifications faites dans le TOML sont perdues : le formulaire réécrit le format à partir de ce qu’il contient.
formatwizard-back-to-form = Revenir au formulaire
formatwizard-miss-name = Ses fichiers n’acceptent pas le nom { $file }. Ajoutez-le à Fichiers.
formatwizard-miss-not-the-stores = Ce à quoi il reconnaît les fichiers de la boutique ne se trouve pas dans ce fichier. Modifiez Reconnaître par.
formatwizard-miss-no-title = Il ne trouve aucun titre dans ce fichier. Désignez le titre.
formatwizard-miss-short-name = pas son nom
formatwizard-miss-short-not-the-stores = pas de la boutique
formatwizard-miss-short-no-title = pas de titre
formatwizard-saved-packs =
    { $file } enregistré. { $packs ->
        [one] { $packs } pack est
       *[other] { $packs } packs sont
    } en cours de relecture.
formatwizard-saved = { $file } enregistré.
formatwizard-save-failed = Impossible d’enregistrer le format : { $error }
formatwizard-zip-failed = Impossible de créer le zip : { $error }
formatwizard-stop-title = Arrêter l’apprentissage de ce fichier ?
formatwizard-stop-body = Le format n’est pas enregistré.
formatwizard-stop = Arrêter
formatwizard-label = Assistant de format
formatwizard-changing = Modification de { $name }
formatwizard-starting-from = À partir de { $name }
formatwizard-title = Apprendre à Hoard le fichier d’une boutique
formatwizard-place-1 = { "" }
formatwizard-place-2 = { " " }dans{ " " }
formatwizard-place-3 = { "" }
formatwizard-edit-toml = Modifier en TOML
formatwizard-file-reference = Le fichier, pour référence.
formatwizard-click-json = Cliquez sur { $field } dans le fichier.
formatwizard-click-text = Cliquez sur { $field } dans le fichier, ou sélectionnez-le.
formatwizard-pick-first = Choisissez un champ à droite, puis cliquez sur sa valeur ici.
formatwizard-loading = Chargement…
formatwizard-the-format = Le format
formatwizard-syntax-before = La syntaxe est décrite dans{ " " }
formatwizard-syntax-after = .
formatwizard-store = Boutique
formatwizard-store-placeholder = Le nom de la boutique, comme Kenney
formatwizard-files = Fichiers
formatwizard-recognize = Reconnaître par
formatwizard-recognize-json = Une clé que seuls les fichiers de la boutique ont
formatwizard-recognize-text = Une ligne que seuls les fichiers de la boutique ont
formatwizard-recognize-hint = Une possibilité par ligne ; n’importe laquelle suffit. Nécessaire quand d’autres fichiers portent le même nom.
formatwizard-seller = Vendeur
formatwizard-seller-placeholder = Quand le fichier ne le dit pas
formatwizard-tags = Étiquettes
formatwizard-tags-placeholder = Pour tout ce qu’il décrit, séparées par des virgules
formatwizard-fields = Champs
formatwizard-selector = Sélecteur du champ { $field }
formatwizard-clear = Effacer le champ { $field }
formatwizard-pick = Désigner
formatwizard-what-it-reads = Ce qu’il lit
formatwizard-read-title = Titre
formatwizard-read-seller = Vendeur
formatwizard-read-license = Licence
formatwizard-read-url = Page de boutique
formatwizard-read-kind = Type
formatwizard-read-tags = Étiquettes
formatwizard-read-origin = Lieu de création
formatwizard-read-size = Taille
formatwizard-also = { $format } lit aussi ce fichier. Ce format-ci serait essayé en premier.
formatwizard-library =
    { $files ->
        [one] { $files } fichier
       *[other] { $files } fichiers
    } dans { $packs ->
        [one] { $packs } pack
       *[other] { $packs } packs
    } { $files ->
        [one] porte
       *[other] portent
    } un nom qu’il accepte. Sur { $tried } essayés, il en lit { $read }.
formatwizard-library-none = Aucun fichier de la bibliothèque ne porte un nom qu’il accepte.
formatwizard-share = Le partager
formatwizard-share-hint = Un zip du format, avec ce fichier comme exemple et ce que l’exemple donne à la lecture, organisé comme le dépôt de Hoard, à envoyer pour que tout le monde en profite. Retirez d’abord de l’exemple tout ce qui ne doit pas être partagé.
formatwizard-sample = Exemple à partager
formatwizard-download = Télécharger { $file }
formatwizard-close = Fermer
formatwizard-use = L’utiliser ici
formatwizard-cancel = Annuler
settings-unreal-editor-what = Installe Unreal Editor pour le processus d’exportation. Exécutez-le séparément pour exporter des ressources.
settings-unreal-download = Télécharge une fois l’éditeur Linux x86-64 d’Epic dans le dossier d’outils de Hoard. Il occupe des dizaines de Go et nécessite un compte GitHub accepté par Epic.
settings-unreal-github-step-link-go = Comment les associer
settings-unreal-github-step-token = Créez un jeton GitHub (classic) avec seulement read:packages coché.
settings-unreal-github-step-token-go = Créer un jeton
settings-unreal-github-user = Nom d’utilisateur GitHub
settings-unreal-github-token = Jeton
settings-unreal-github-token-kept = Enregistré (saisissez-en un nouveau pour le remplacer)
settings-unreal-github-save = Enregistrer la connexion
settings-unreal-github-forget = Déconnecter
settings-unreal-github-saved = Connexion GitHub enregistrée ; nouvelle tentative de téléchargement
settings-unreal-github-forgotten = GitHub déconnecté
settings-unreal-github-from-server = Ce serveur a aussi reçu sa propre connexion (GHCR_TOKEN_FILE ou DOCKER_CONFIG) ; elle sert quand aucune n’est enregistrée ici.
settings-unreal-github-connect = Connecter GitHub…
settings-unreal-github-connect-title = Connecter GitHub
settings-unreal-github-connect-why = Epic partage Unreal Editor avec les comptes GitHub liés à Epic Games. Hoard demande seulement à lire les paquets.
settings-unreal-github-sign-in = Se connecter avec GitHub
settings-unreal-github-enter-code = Après le clic, GitHub s’ouvre dans un nouvel onglet. Saisissez-y ce code.
settings-unreal-github-waiting = En attente de GitHub…
settings-unreal-github-expired = Ce code a expiré.
settings-unreal-github-denied = La connexion a été annulée sur GitHub.
settings-unreal-github-failed = Impossible d’obtenir un code de GitHub.
settings-unreal-github-again = Obtenir un nouveau code
settings-unreal-github-before = Avant de commencer
settings-unreal-github-before-what = Liez GitHub à votre compte Epic Games et acceptez l’invitation EpicGames que GitHub vous envoie par e-mail.
settings-unreal-github-use-token = Utiliser plutôt un jeton
settings-unreal-github-use-sign-in = Se connecter plutôt avec GitHub
settings-unreal-github-no-password = Aucun mot de passe ne parvient à Hoard
# Shown once GitHub is connected; $user is the GitHub user name.
settings-unreal-github-connected = Connecté en tant que { $user } ; nouvel essai du téléchargement
# $user is the GitHub user name Unreal Editor is downloaded as.
settings-unreal-github-as = Connecté en tant que { $user }
settings-unreal-github-as-what = Unreal Editor est téléchargé avec ce compte GitHub.
settings-unreal-github-none = GitHub n’est pas connecté
settings-unreal-github-none-what = Nécessaire pour télécharger l’éditeur.
settings-unreal-github-change = Changer…
settings-unreal-image = Version ou image Unreal
settings-editor-path = Indiquez le chemin de l’exécutable de votre installation de { $tool }. L’installation automatique est disponible uniquement dans Docker.
error-unreal-image = Indiquez une version d’Unreal 5, une étiquette de version dev/dev-slim d’Epic ou une référence d’image Epic SHA-256.
error-github-no-app = Ce serveur n’a pas d’application GitHub pour se connecter ; utilisez plutôt un jeton.
error-github-unreachable = Impossible de joindre GitHub.
# $reason is GitHub's own explanation, in English.
error-github-refused = GitHub a refusé la connexion : { $reason }
