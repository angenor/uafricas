<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Mon compte"
        :sous-titre="profil ? profil.email : undefined"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Mon compte' }]">
        <template v-if="profil" #action>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-eye" :vers="`/profil/${profil.id}`">
            Voir mon profil public
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div v-if="chargement" class="flex flex-col gap-6">
      <div class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
      <div class="h-96 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="profil" class="flex flex-col gap-6">
      <!-- ═══ Identité ═══
           Le bandeau dégradé orange→vert qui coiffait cette carte est retiré :
           il n'appartient à aucun jeton de la refonte, et il ne servait que de
           support à un avatar posé à cheval dessus. -->
      <section class="flex flex-col items-center gap-5 rounded-[10px] border border-af-bordure bg-white p-6 text-center sm:flex-row sm:items-start sm:text-left">
        <!-- La photo se change au survol de l'avatar. `focus-within` est
             indispensable : sans lui, le bouton reste invisible pour qui
             navigue au clavier, et donc inatteignable. -->
        <div class="group relative shrink-0">
          <AfricansAvatar :nom="nomComplet" :src="photoComplete" :taille="112" />
          <label
            class="absolute right-0 bottom-0 grid size-9 cursor-pointer place-items-center rounded-full bg-af-vert text-white opacity-0 shadow-md transition group-hover:opacity-100 focus-within:opacity-100"
            title="Changer ma photo"
          >
            <font-awesome-icon icon="fa-solid fa-image" class="text-xs" />
            <span class="sr-only">Changer ma photo de profil</span>
            <input
              type="file"
              accept="image/jpeg,image/png,image/webp"
              class="hidden"
              @change="onPhotoChange"
            />
          </label>
        </div>

        <div class="min-w-0 flex-1">
          <h1 class="text-[24px]/[1.3] font-bold text-af-encre">{{ nomComplet }}</h1>
          <p class="text-[14px]/[1.4] text-af-corps">{{ profil.email }}</p>

          <div class="mt-3 flex flex-wrap items-center justify-center gap-2 sm:justify-start">
            <span
              class="inline-flex items-center gap-1.5 rounded px-3 py-1 text-[12px]/[1.4] font-bold"
              :class="badgeEtatClasses"
            >
              <span class="size-2 rounded-full" :class="pointEtatClasses" />
              {{ etatLabel }}
            </span>
            <AfricansEtiquette v-for="role in profil.roles" :key="role">{{ role }}</AfricansEtiquette>
            <span class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
              <font-awesome-icon icon="fa-solid fa-calendar" />
              Membre depuis {{ dateInscription }}
            </span>
          </div>

          <!-- Accès direct à la gestion des supports détenus : la section
               existe, mais elle siège au septième rang du menu. Rien ne
               s'affiche pour qui ne détient aucun support. -->
          <div v-if="nombreSupports > 0" class="mt-4 flex justify-center sm:justify-start">
            <AfricansBouton variante="secondaire" icone="fa-solid fa-tv" @click="allerAuxSupports">
              Gérer mes supports médias ({{ nombreSupports }})
            </AfricansBouton>
          </div>
        </div>
      </section>

      <!-- ═══ Messages succès / erreur ═══ -->
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 -translate-y-2"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 translate-y-0"
        leave-to-class="opacity-0 -translate-y-2"
      >
        <div
          v-if="profilComposable.success.value"
          class="flex items-center gap-2 rounded-[10px] border border-af-vert/30 bg-af-vert/5 px-4 py-3 text-[14px]/[1.4] text-af-vert"
        >
          <font-awesome-icon icon="fa-solid fa-circle-check" />
          {{ profilComposable.success.value }}
        </div>
      </Transition>

      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 -translate-y-2"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 translate-y-0"
        leave-to-class="opacity-0 -translate-y-2"
      >
        <div
          v-if="profilComposable.error.value"
          class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
        >
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
          {{ profilComposable.error.value }}
        </div>
      </Transition>

      <!-- ═══ Section retenue ═══
           Le menu qui la commande est passé dans le rail : le gabarit occupe
           déjà la colonne de gauche avec la navigation de la plateforme, et
           deux menus verticaux côte à côte ne se distinguent plus l'un de
           l'autre. -->
      <div ref="zoneOnglets" class="min-w-0 scroll-mt-28 rounded-[10px] border border-af-bordure bg-white p-6">

            <!-- ─── Onglet Informations ─── -->
            <div v-if="ongletActif === 'informations'" class="space-y-6">
              <div class="flex items-center justify-between mb-2">
                <h2 class="text-lg font-semibold text-af-encre">Informations personnelles</h2>
                <button
                  v-if="!modeEdition"
                  class="flex items-center gap-2 px-4 py-2 text-sm text-af-chocolat hover:bg-af-chocolat/5 rounded-lg transition-colors"
                  @click="activerEdition"
                >
                  <font-awesome-icon icon="fa-solid fa-pen-to-square" />
                  Modifier
                </button>
              </div>

              <!-- Mode Lecture -->
              <div v-if="!modeEdition" class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfilChampLecture label="Prénom" :valeur="profil.prenom" icon="fa-solid fa-user" />
                <ProfilChampLecture label="Nom" :valeur="profil.nom" icon="fa-solid fa-user" />
                <ProfilChampLecture label="Email" :valeur="profil.email" icon="fa-solid fa-envelope" />
                <ProfilChampLecture label="Téléphone" :valeur="profil.telephone" icon="fa-solid fa-phone" />
                <ProfilChampLecture label="Genre" :valeur="genreLabel" icon="fa-solid fa-user" />
                <ProfilChampLecture label="Date de naissance" :valeur="dateNaissanceFormatee" icon="fa-solid fa-calendar" />
                <ProfilChampLecture label="Fonction" :valeur="profil.fonction" icon="fa-solid fa-briefcase" />
                <div class="md:col-span-2">
                  <ProfilChampLecture label="Biographie" :valeur="profil.biographie" icon="fa-solid fa-align-left" />
                </div>
              </div>

              <!-- Mode Edition -->
              <form v-else @submit.prevent="sauvegarderProfil" class="space-y-5">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Prénom</label>
                    <input
                      v-model="formulaire.prenom"
                      type="text"
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Nom</label>
                    <input
                      v-model="formulaire.nom"
                      type="text"
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Téléphone</label>
                    <input
                      v-model="formulaire.telephone"
                      type="tel"
                      placeholder="+225 00 00 00 00"
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Genre</label>
                    <select
                      v-model="formulaire.genre"
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    >
                      <option value="non_precise">Non précisé</option>
                      <option value="homme">Homme</option>
                      <option value="femme">Femme</option>
                    </select>
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Date de naissance</label>
                    <input
                      v-model="formulaire.date_naissance"
                      type="date"
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Fonction</label>
                    <input
                      v-model="formulaire.fonction"
                      type="text"
                      placeholder="Ex: Developpeur, Enseignant..."
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                </div>
                <div class="space-y-2">
                  <label class="text-sm font-medium text-af-corps block">Biographie</label>
                  <textarea
                    v-model="formulaire.biographie"
                    rows="4"
                    placeholder="Parlez de vous..."
                    class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white resize-none"
                  ></textarea>
                </div>

                <!-- Boutons actions -->
                <div class="flex items-center justify-end gap-3 pt-2">
                  <button
                    type="button"
                    class="px-5 py-2.5 text-sm text-af-corps hover:bg-af-fond rounded-lg transition-colors"
                    @click="annulerEdition"
                  >
                    Annuler
                  </button>
                  <button
                    type="submit"
                    class="flex items-center gap-2 px-5 py-2.5 text-sm bg-af-degrade text-white font-medium rounded-lg hover:shadow-lg transition-all duration-300 disabled:opacity-50"
                    :disabled="profilComposable.loading.value"
                  >
                    <font-awesome-icon
                      :icon="profilComposable.loading.value ? 'fa-solid fa-spinner' : 'fa-solid fa-floppy-disk'"
                      :class="{ 'animate-spin': profilComposable.loading.value }"
                    />
                    Enregistrer
                  </button>
                </div>
              </form>
            </div>

            <!-- ─── Onglet Localisation ─── -->
            <div v-if="ongletActif === 'localisation'" class="space-y-6">
              <div class="flex items-center justify-between mb-2">
                <h2 class="text-lg font-semibold text-af-encre">Localisation et préférences</h2>
                <button
                  v-if="!modeEditionLocalisation"
                  class="flex items-center gap-2 px-4 py-2 text-sm text-af-chocolat hover:bg-af-chocolat/5 rounded-lg transition-colors"
                  @click="activerEditionLocalisation"
                >
                  <font-awesome-icon icon="fa-solid fa-pen-to-square" />
                  Modifier
                </button>
              </div>

              <!-- Mode Lecture -->
              <div v-if="!modeEditionLocalisation" class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfilChampLecture label="Ville" :valeur="profil.ville" icon="fa-solid fa-location-dot" />
                <ProfilChampLecture label="Localite / Quartier" :valeur="profil.localite" icon="fa-solid fa-map-pin" />
                <ProfilChampLecture label="Langue preferee" :valeur="langueLabel" icon="fa-solid fa-language" />
              </div>

              <!-- Mode Edition -->
              <form v-else @submit.prevent="sauvegarderLocalisation" class="space-y-5">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Ville</label>
                    <input
                      v-model="formulaireLocalisation.ville"
                      type="text"
                      placeholder="Ex: Abidjan, Dakar..."
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Localite / Quartier</label>
                    <input
                      v-model="formulaireLocalisation.localite"
                      type="text"
                      placeholder="Ex: Cocody, Plateau..."
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Langue preferee</label>
                    <select
                      v-model="formulaireLocalisation.langue_preferee"
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    >
                      <option value="fr">Francais</option>
                      <option value="en">Anglais</option>
                      <option value="pt">Portugais</option>
                      <option value="ar">Arabe</option>
                      <option value="sw">Swahili</option>
                    </select>
                  </div>
                </div>

                <div class="flex items-center justify-end gap-3 pt-2">
                  <button
                    type="button"
                    class="px-5 py-2.5 text-sm text-af-corps hover:bg-af-fond rounded-lg transition-colors"
                    @click="modeEditionLocalisation = false"
                  >
                    Annuler
                  </button>
                  <button
                    type="submit"
                    class="flex items-center gap-2 px-5 py-2.5 text-sm bg-af-degrade text-white font-medium rounded-lg hover:shadow-lg transition-all duration-300 disabled:opacity-50"
                    :disabled="profilComposable.loading.value"
                  >
                    <font-awesome-icon
                      :icon="profilComposable.loading.value ? 'fa-solid fa-spinner' : 'fa-solid fa-floppy-disk'"
                      :class="{ 'animate-spin': profilComposable.loading.value }"
                    />
                    Enregistrer
                  </button>
                </div>
              </form>
            </div>

            <!-- ─── Onglet Africonnect ─── -->
            <div v-if="ongletActif === 'retrouve-amis'" class="space-y-6">
              <h2 class="text-lg font-semibold text-af-encre mb-2">Africonnect</h2>

              <!-- Statut trouvable -->
              <div class="bg-af-fond rounded-lg p-5">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div
                      class="w-9 h-9 rounded-lg flex items-center justify-center"
                      :class="profil.est_trouvable ? 'bg-af-vert/10 text-af-vert' : 'bg-af-bordure text-af-atone-2'"
                    >
                      <font-awesome-icon icon="fa-solid fa-eye" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-sm font-medium text-af-encre">Profil trouvable</p>
                      <p class="text-xs text-af-atone">
                        {{ profil.est_trouvable ? 'Votre profil est visible pour le matching' : 'Votre profil est masque' }}
                      </p>
                    </div>
                  </div>
                  <span
                    class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-xs font-medium"
                    :class="profil.est_trouvable ? 'bg-af-vert/10 text-af-vert' : 'bg-af-fond text-af-atone'"
                  >
                    {{ profil.est_trouvable ? 'Actif' : 'Inactif' }}
                  </span>
                </div>
              </div>

              <!-- Parcours -->
              <div class="border border-af-bordure rounded-lg p-5">
                <div class="flex items-center gap-3 mb-4">
                  <div class="w-9 h-9 rounded-lg bg-af-chocolat/10 text-af-chocolat flex items-center justify-center">
                    <font-awesome-icon icon="fa-solid fa-route" class="text-sm" />
                  </div>
                  <div>
                    <h3 class="text-sm font-semibold text-af-encre">Mon parcours</h3>
                    <p class="text-xs text-af-atone">{{ parcoursRetrouvAmis.length }} entree(s) de parcours</p>
                  </div>
                </div>

                <!-- Liste parcours -->
                <div v-if="parcoursRetrouvAmis.length > 0" class="space-y-2 mb-4">
                  <div
                    v-for="p in parcoursRetrouvAmis"
                    :key="p.id"
                    class="flex items-center gap-3 px-3 py-2 bg-af-fond rounded-lg text-sm"
                  >
                    <font-awesome-icon
                      :icon="p.type_entree === 'ecole' ? 'fa-solid fa-graduation-cap' : p.type_entree === 'ville_residence' ? 'fa-solid fa-building' : 'fa-solid fa-briefcase'"
                      class="text-af-atone-2 w-4"
                    />
                    <span class="font-medium text-af-corps">{{ p.nom }}</span>
                    <span v-if="p.ville" class="text-af-atone-2">- {{ p.ville }}</span>
                    <span v-if="p.periode_debut" class="text-af-atone-2 ml-auto text-xs">
                      {{ p.periode_debut }}{{ p.periode_fin ? ` - ${p.periode_fin}` : '' }}
                    </span>
                  </div>
                </div>

                <NuxtLink
                  to="/retrouve-amis"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm text-af-chocolat hover:bg-af-chocolat/5 rounded-lg transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-arrow-right" class="text-xs" />
                  Gérer dans Africonnect
                </NuxtLink>
              </div>
            </div>

            <!-- ─── Onglet Sécurité ─── -->
            <div v-if="ongletActif === 'securite'" class="space-y-6">
              <h2 class="text-lg font-semibold text-af-encre mb-2">Sécurité du compte</h2>

              <!-- Infos compte -->
              <div class="bg-af-fond rounded-lg p-5 space-y-4">
                <h3 class="text-sm font-semibold text-af-corps uppercase tracking-wide">Informations du compte</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-af-vert/10 text-af-vert flex items-center justify-center">
                      <font-awesome-icon icon="fa-solid fa-envelope" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-af-atone">Email vérifié</p>
                      <p class="text-sm font-medium" :class="profil.email_verifie ? 'text-af-vert' : 'text-af-live'">
                        {{ profil.email_verifie ? 'Oui' : 'Non' }}
                      </p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-af-chocolat/10 text-af-chocolat flex items-center justify-center">
                      <font-awesome-icon icon="fa-solid fa-calendar" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-af-atone">Date d'inscription</p>
                      <p class="text-sm font-medium text-af-corps">{{ dateInscriptionComplete }}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-af-chocolat/10 text-af-chocolat flex items-center justify-center">
                      <font-awesome-icon icon="fa-solid fa-clock-rotate-left" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-af-atone">Derniere connexion</p>
                      <p class="text-sm font-medium text-af-corps">{{ derniereConnexionFormatee }}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg flex items-center justify-center"
                      :class="profil.etat === 'actif' ? 'bg-af-vert/10 text-af-vert' : 'bg-af-chocolat/10 text-af-chocolat'"
                    >
                      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-af-atone">Etat du compte</p>
                      <p class="text-sm font-medium" :class="profil.etat === 'actif' ? 'text-af-vert' : 'text-af-chocolat'">
                        {{ etatLabel }}
                      </p>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Changer mot de passe -->
              <div class="border border-af-bordure rounded-lg p-5">
                <div class="flex items-center gap-3 mb-4">
                  <div class="w-9 h-9 rounded-lg bg-af-live/10 text-af-live flex items-center justify-center">
                    <font-awesome-icon icon="fa-solid fa-lock" class="text-sm" />
                  </div>
                  <h3 class="text-sm font-semibold text-af-encre">Changer le mot de passe</h3>
                </div>

                <form @submit.prevent="handleChangerMotDePasse" class="space-y-4">
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-af-corps block">Mot de passe actuel</label>
                    <input
                      v-model="formulaireMdp.ancien_mot_de_passe"
                      type="password"
                      required
                      class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                    />
                  </div>
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="space-y-2">
                      <label class="text-sm font-medium text-af-corps block">Nouveau mot de passe</label>
                      <input
                        v-model="formulaireMdp.nouveau_mot_de_passe"
                        type="password"
                        required
                        minlength="6"
                        class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                      />
                    </div>
                    <div class="space-y-2">
                      <label class="text-sm font-medium text-af-corps block">Confirmer le mot de passe</label>
                      <input
                        v-model="formulaireMdp.confirmation_mot_de_passe"
                        type="password"
                        required
                        minlength="6"
                        class="w-full px-4 py-3 border border-af-bordure rounded-lg focus:ring-2 focus:ring-af-vert focus:border-transparent transition-all duration-300 bg-af-fond hover:bg-white"
                      />
                    </div>
                  </div>
                  <div class="flex justify-end">
                    <button
                      type="submit"
                      class="flex items-center gap-2 px-5 py-2.5 text-sm bg-af-live text-white font-medium rounded-lg hover:opacity-90 transition-colors disabled:opacity-50"
                      :disabled="profilComposable.loading.value"
                    >
                      <font-awesome-icon
                        :icon="profilComposable.loading.value ? 'fa-solid fa-spinner' : 'fa-solid fa-lock'"
                        :class="{ 'animate-spin': profilComposable.loading.value }"
                      />
                      Changer le mot de passe
                    </button>
                  </div>
                </form>
              </div>
            </div>

            <!-- ─── Onglet Bibliothèque Humaine ─── -->
            <div v-if="ongletActif === 'bibliotheque-humaine'" class="space-y-6">
              <h2 class="text-lg font-semibold text-af-encre">Bibliothèque Humaine</h2>

              <div v-if="chargementDemande" class="flex justify-center py-8">
                <font-awesome-icon icon="fa-solid fa-spinner" class="text-2xl text-af-chocolat animate-spin" />
              </div>

              <div v-else-if="!maDemande" class="text-center py-10">
                <font-awesome-icon icon="fa-solid fa-book-open" class="text-4xl text-af-atone-2 mb-3" />
                <p class="text-af-atone text-sm mb-4">Vous n'avez pas encore soumis de demande pour rejoindre la Bibliothèque Humaine.</p>
                <NuxtLink
                  to="/bibliotheque/humaine"
                  class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-af-degrade text-white font-medium rounded-lg hover:shadow-lg transition-all duration-300"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  Soumettre une demande
                </NuxtLink>
              </div>

              <div v-else class="space-y-4">
                <!-- Badge statut -->
                <div
                  class="flex items-center gap-3 p-4 rounded-lg border"
                  :class="{
                    'bg-af-chocolat/5 border-af-chocolat/20': maDemande.statut === 'en_attente',
                    'bg-af-vert/5 border-af-vert/30': maDemande.statut === 'valide',
                    'bg-af-live/5 border-af-live/30': maDemande.statut === 'rejete',
                  }"
                >
                  <font-awesome-icon
                    :icon="maDemande.statut === 'valide' ? 'fa-solid fa-circle-check' : maDemande.statut === 'rejete' ? 'fa-solid fa-circle-xmark' : 'fa-solid fa-clock'"
                    class="text-xl"
                    :class="{
                      'text-af-chocolat': maDemande.statut === 'en_attente',
                      'text-af-vert': maDemande.statut === 'valide',
                      'text-af-live': maDemande.statut === 'rejete',
                    }"
                  />
                  <div class="flex-1">
                    <p
                      class="font-semibold text-sm"
                      :class="{
                        'text-af-chocolat': maDemande.statut === 'en_attente',
                        'text-af-vert': maDemande.statut === 'valide',
                        'text-af-live': maDemande.statut === 'rejete',
                      }"
                    >
                      {{
                        maDemande.statut === 'en_attente' ? 'En attente de validation'
                        : maDemande.statut === 'valide' ? 'Demande validée'
                        : 'Demande rejetée'
                      }}
                    </p>
                    <p class="text-xs text-af-atone mt-0.5">
                      Soumis le {{ new Date(maDemande.createdAt).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) }}
                    </p>
                  </div>
                </div>

                <!-- Commentaire admin si rejeté -->
                <div v-if="maDemande.statut === 'rejete' && maDemande.commentaireAdmin" class="p-4 bg-af-live/5 border border-af-live/30 rounded-lg">
                  <p class="text-xs font-semibold text-af-live uppercase tracking-wide mb-1">Motif du rejet</p>
                  <p class="text-sm text-af-live">{{ maDemande.commentaireAdmin }}</p>
                </div>

                <!-- Re-soumettre si rejeté -->
                <div v-if="maDemande.statut === 'rejete'" class="text-center pt-2">
                  <NuxtLink
                    to="/bibliotheque/humaine"
                    class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-af-degrade text-white font-medium rounded-lg hover:shadow-lg transition-all duration-300"
                  >
                    <font-awesome-icon icon="fa-solid fa-rotate-right" />
                    Soumettre une nouvelle demande
                  </NuxtLink>
                </div>

                <!-- Résumé -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                  <div>
                    <p class="text-xs font-semibold text-af-atone uppercase tracking-wide mb-1">Fonction déclarée</p>
                    <p class="text-sm text-af-corps">{{ maDemande.fonction }}</p>
                  </div>
                  <div v-if="maDemande.pays">
                    <p class="text-xs font-semibold text-af-atone uppercase tracking-wide mb-1">Territoire</p>
                    <p class="text-sm text-af-corps">{{ maDemande.pays }}</p>
                  </div>
                  <div v-if="maDemande.specialites.length > 0" class="md:col-span-2">
                    <p class="text-xs font-semibold text-af-atone uppercase tracking-wide mb-1">Spécialités</p>
                    <div class="flex flex-wrap gap-1">
                      <span
                        v-for="s in maDemande.specialites"
                        :key="s"
                        class="inline-block px-2 py-0.5 bg-af-fond text-af-corps text-xs rounded-full"
                      >{{ s }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- ─── Onglet Expertise ─── -->
            <div v-if="ongletActif === 'expertise'" class="space-y-6">
              <h2 class="text-lg font-semibold text-af-encre">Mon expertise</h2>

              <div v-if="chargementExpertise" class="flex justify-center py-8">
                <font-awesome-icon icon="fa-solid fa-spinner" class="text-2xl text-af-chocolat animate-spin" />
              </div>

              <div v-else-if="!maCandidatureExpert" class="text-center py-10">
                <font-awesome-icon icon="fa-solid fa-user-tie" class="text-4xl text-af-atone-2 mb-3" />
                <p class="text-af-atone text-sm mb-4">Vous n'avez pas encore soumis de demande pour devenir expert.</p>
                <NuxtLink
                  to="/devenir-expert"
                  class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-af-degrade text-white font-medium rounded-lg hover:shadow-lg transition-all duration-300"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  Apporter mon expertise
                </NuxtLink>
              </div>

              <div v-else class="space-y-4">
                <!-- Badge statut -->
                <div
                  class="flex items-center gap-3 p-4 rounded-lg border"
                  :class="{
                    'bg-af-chocolat/5 border-af-chocolat/20': maCandidatureExpert.statut === 'en_attente',
                    'bg-af-vert/5 border-af-vert/30': maCandidatureExpert.statut === 'valide',
                    'bg-af-live/5 border-af-live/30': maCandidatureExpert.statut === 'refuse',
                  }"
                >
                  <font-awesome-icon
                    :icon="maCandidatureExpert.statut === 'valide' ? 'fa-solid fa-circle-check' : maCandidatureExpert.statut === 'refuse' ? 'fa-solid fa-circle-xmark' : 'fa-solid fa-clock'"
                    class="text-xl"
                    :class="{
                      'text-af-chocolat': maCandidatureExpert.statut === 'en_attente',
                      'text-af-vert': maCandidatureExpert.statut === 'valide',
                      'text-af-live': maCandidatureExpert.statut === 'refuse',
                    }"
                  />
                  <div class="flex-1">
                    <p
                      class="font-semibold text-sm"
                      :class="{
                        'text-af-chocolat': maCandidatureExpert.statut === 'en_attente',
                        'text-af-vert': maCandidatureExpert.statut === 'valide',
                        'text-af-live': maCandidatureExpert.statut === 'refuse',
                      }"
                    >
                      {{
                        maCandidatureExpert.statut === 'en_attente' ? 'En attente de validation'
                        : maCandidatureExpert.statut === 'valide' ? 'Demande validée'
                        : 'Demande refusée'
                      }}
                    </p>
                    <p class="text-xs text-af-atone mt-0.5">
                      Soumis le {{ new Date(maCandidatureExpert.createdAt).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) }}
                    </p>
                  </div>
                </div>

                <!-- Commentaire admin si refusé -->
                <div v-if="maCandidatureExpert.statut === 'refuse' && maCandidatureExpert.commentaireAdmin" class="p-4 bg-af-live/5 border border-af-live/30 rounded-lg">
                  <p class="text-xs font-semibold text-af-live uppercase tracking-wide mb-1">Motif du refus</p>
                  <p class="text-sm text-af-live">{{ maCandidatureExpert.commentaireAdmin }}</p>
                </div>

                <!-- Re-soumettre si refusé -->
                <div v-if="maCandidatureExpert.statut === 'refuse'" class="text-center pt-2">
                  <NuxtLink
                    to="/devenir-expert"
                    class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-af-degrade text-white font-medium rounded-lg hover:shadow-lg transition-all duration-300"
                  >
                    <font-awesome-icon icon="fa-solid fa-rotate-left" />
                    Soumettre une nouvelle demande
                  </NuxtLink>
                </div>

                <!-- Lien fiche publique si validé -->
                <div v-if="maCandidatureExpert.statut === 'valide' && profil" class="text-center pt-2">
                  <NuxtLink
                    :to="`/experts/${profil.id}`"
                    class="inline-flex items-center gap-2 px-5 py-2.5 text-sm border border-af-vert text-af-vert font-medium rounded-lg hover:bg-af-vert/5 transition-all duration-300"
                  >
                    <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" />
                    Voir ma fiche d'expert
                  </NuxtLink>
                </div>

                <!-- Résumé -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                  <div>
                    <p class="text-xs font-semibold text-af-atone uppercase tracking-wide mb-1">Domaine</p>
                    <p class="text-sm text-af-corps">{{ maCandidatureExpert.domaine }}</p>
                  </div>
                  <div>
                    <p class="text-xs font-semibold text-af-atone uppercase tracking-wide mb-1">Années d'expérience</p>
                    <p class="text-sm text-af-corps">{{ maCandidatureExpert.nbAnneesExperience }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- ─── Onglet Mes échanges sabbatiques ─── -->
            <div v-if="ongletActif === 'mes-echanges'" class="space-y-6">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-af-encre">Mes projets d'échange</h2>
                <NuxtLink
                  to="/echanges-sabbatiques/proposer"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-af-vert text-white font-medium rounded-lg hover:bg-af-vert/90 transition-all"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  <span class="hidden sm:inline">Proposer</span>
                </NuxtLink>
              </div>
              <p class="text-sm text-af-atone -mt-3">
                Gérez les candidatures reçues et sélectionnez le candidat final pour chaque projet.
              </p>
              <SabbatiqueMesEchanges />
            </div>

            <!-- ─── Onglet Mes événements ─── -->
            <div v-if="ongletActif === 'mes-evenements'" class="space-y-6">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-af-encre">Mes événements</h2>
                <NuxtLink
                  to="/evenements/liste"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-af-vert text-white font-medium rounded-lg hover:bg-af-vert/90 transition-all"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  <span class="hidden sm:inline">Proposer</span>
                </NuxtLink>
              </div>
              <p class="text-sm text-af-atone -mt-3">
                Gérez vos événements : modifiez, supprimez, consultez les inscrits et démarrez vos diffusions en direct.
              </p>
              <EvenementsMesEvenements />
            </div>

            <!-- ─── Onglet Mes points ─── -->
            <div v-if="ongletActif === 'mes-points'" class="space-y-6">
              <EngagementMesPointsPanel />
            </div>

            <!-- ─── Onglet Mes supports médias ─── -->
            <div v-if="ongletActif === 'mes-supports'" class="space-y-6">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-af-encre">Mes supports médias</h2>
                <NuxtLink
                  to="/mon-compte/invitations-medias"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-af-vert text-white font-medium rounded-lg hover:bg-af-vert/90 transition-all"
                >
                  <font-awesome-icon icon="fa-solid fa-envelope" />
                  <span class="hidden sm:inline">Invitations</span>
                </NuxtLink>
              </div>
              <p class="text-sm text-af-atone -mt-3">
                Les chaînes et stations que vous détenez : bâtissez leur grille de
                programmation, arbitrez les idées et demandes d'animation reçues,
                et gérez votre équipe.
              </p>
              <MediaMesSupports />
            </div>
      </div>
    </div>

    <template #rail>
      <!-- Sections du compte. Groupées : dix entrées d'affilée ne se
           parcourent pas, on les balaie. Le regroupement dit aussi *pourquoi*
           on chercherait chaque section : réglages du compte, statut d'une
           candidature, ou ce qu'on anime sur la plateforme. -->
      <AfricansPanneau titre="Mon compte" icone="fa-solid fa-gear">
        <nav aria-label="Sections de mon compte" class="flex flex-col gap-5">
          <div v-for="groupe in GROUPES_ONGLETS" :key="groupe.titre">
            <p class="px-3 pb-1.5 text-[12px]/[1.4] font-bold tracking-wider text-af-atone-2 uppercase">
              {{ groupe.titre }}
            </p>
            <button
              v-for="tab in groupe.onglets"
              :key="tab.id"
              type="button"
              class="flex w-full cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 text-left text-[14px]/[1.4] transition"
              :class="ongletActif === tab.id
                ? 'bg-af-chocolat/15 font-bold text-af-chocolat'
                : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
              :aria-current="ongletActif === tab.id ? 'page' : undefined"
              @click="selectionnerOnglet(tab.id)"
            >
              <font-awesome-icon :icon="tab.icon" class="w-4 shrink-0 text-center" />
              <span class="min-w-0 truncate">{{ tab.label }}</span>
              <!-- Combien de supports on détient, lisible sans ouvrir la section -->
              <span
                v-if="tab.id === 'mes-supports' && nombreSupports > 0"
                class="ml-auto inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-af-chocolat px-1.5 text-[11px] font-bold text-white"
              >
                {{ nombreSupports }}
              </span>
            </button>
          </div>
        </nav>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
import type { Profil, ModifierProfilForm } from '~/composables/useProfil'
import type { MaDemandeAPI } from '~/composables/useBibliothequeHumaine'
import type { MaCandidatureAPI } from '~/composables/useExperts'

useHead({
  title: 'Mon profil - AfricanS',
})

definePageMeta({ layout: false })

useAOS()

const { isAuthenticated, redirigerVersConnexion } = useAuth()
const profilComposable = useProfil()

const profil = ref<Profil | null>(null)
const chargement = ref(true)
const route = useRoute()

/**
 * Onglet d'arrivée.
 *
 * `?onglet=` fait foi quand il est donné. À défaut, `?support=<id>` suffit à
 * désigner l'onglet : c'est ce que portent les passerelles venues des vitrines
 * publiques (« Gérer ma chaîne » dans une section de `/medias/tele`), qui ne
 * connaissent que l'identifiant du support. `MediaMesSupports` s'en sert
 * ensuite pour déplier le bon panneau.
 */
const ongletActif = ref(
  typeof route.query.onglet === 'string'
    ? route.query.onglet
    : route.query.support
      ? 'mes-supports'
      : 'informations',
)
const modeEdition = ref(false)
const modeEditionLocalisation = ref(false)

// Africonnect
const retrouvAmis = useRetrouvAmis()
const parcoursRetrouvAmis = ref<any[]>([])

// Bibliothèque Humaine
const { obtenirMaDemande } = useBibliothequeHumaine()
const maDemande = ref<MaDemandeAPI | null>(null)
const chargementDemande = ref(false)

// Expertise
const { obtenirMaCandidature } = useExperts()
const maCandidatureExpert = ref<MaCandidatureAPI | null>(null)
const chargementExpertise = ref(false)

// ── Supports médias détenus ──
// Comptés au montage pour que l'accès à leur gestion soit visible d'emblée :
// enfoui dans un menu de dix onglets, il ne se trouvait qu'en le cherchant.
const { mesSupports } = useMediaDetention()
const nombreSupports = ref(0)

/** Ancre de la zone d'onglets, pour y amener le raccourci d'en-tête. */
const zoneOnglets = ref<HTMLElement | null>(null)

const allerAuxSupports = () => {
  ongletActif.value = 'mes-supports'
  nextTick(() => zoneOnglets.value?.scrollIntoView({ behavior: 'smooth', block: 'start' }))
}

// ── Sections du compte ──
// Groupées : dix entrées d'affilée ne se parcourent pas, on les balaie. Le
// regroupement dit aussi *pourquoi* on chercherait chaque section, réglages du
// compte, statut d'une candidature, ou ce qu'on anime sur la plateforme.
interface OngletProfil { id: string, label: string, icon: string }

const GROUPES_ONGLETS: { titre: string, onglets: OngletProfil[] }[] = [
  {
    titre: 'Mon compte',
    onglets: [
      { id: 'informations', label: 'Informations', icon: 'fa-solid fa-user' },
      { id: 'localisation', label: 'Localisation', icon: 'fa-solid fa-location-dot' },
      { id: 'securite', label: 'Sécurité', icon: 'fa-solid fa-lock' }],
  },
  {
    titre: 'Mes demandes',
    onglets: [
      { id: 'bibliotheque-humaine', label: 'Bibliothèque', icon: 'fa-solid fa-book-open' },
      { id: 'expertise', label: 'Expertise', icon: 'fa-solid fa-user-tie' },
      { id: 'retrouve-amis', label: 'Africonnect', icon: 'fa-solid fa-users' }],
  },
  {
    titre: 'Ce que j’anime',
    onglets: [
      { id: 'mes-supports', label: 'Mes supports médias', icon: 'fa-solid fa-tv' },
      { id: 'mes-evenements', label: 'Mes événements', icon: 'fa-solid fa-calendar-day' },
      { id: 'mes-echanges', label: 'Mes échanges', icon: 'fa-solid fa-right-left' },
      { id: 'mes-points', label: 'Mes points', icon: 'fa-solid fa-medal' },
    ],
  },
]

/** Liste à plat : la rangée mobile ignore les groupes, faute de place. */
const onglets = GROUPES_ONGLETS.flatMap(g => g.onglets)

const selectionnerOnglet = (id: string) => {
  ongletActif.value = id
}

// ── Formulaires ──
const formulaire = reactive<ModifierProfilForm>({
  prenom: '',
  nom: '',
  telephone: '',
  genre: 'non_precise',
  date_naissance: null,
  fonction: '',
  biographie: '',
})

const formulaireLocalisation = reactive({
  ville: '',
  localite: '',
  langue_preferee: 'fr',
})

const formulaireMdp = reactive({
  ancien_mot_de_passe: '',
  nouveau_mot_de_passe: '',
  confirmation_mot_de_passe: '',
})

// ── Computed ──
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const nomComplet = computed(() => {
  if (!profil.value) return ''
  return `${profil.value.prenom} ${profil.value.nom}`
})

const photoComplete = computed(() => {
  if (!profil.value?.photo_url) return ''
  if (profil.value.photo_url.startsWith('http')) return profil.value.photo_url
  return `${apiBase}${profil.value.photo_url}`
})

const etatLabel = computed(() => {
  const etats: Record<string, string> = {
    actif: 'Actif',
    en_attente: 'En attente',
    bloque: 'Bloque',
    suspendu: 'Suspendu',
  }
  return etats[profil.value?.etat || ''] || profil.value?.etat || ''
})

const badgeEtatClasses = computed(() => {
  switch (profil.value?.etat) {
    case 'actif': return 'bg-af-vert/5 text-af-vert'
    case 'en_attente': return 'bg-af-chocolat/5 text-af-chocolat'
    case 'bloque': return 'bg-af-live/5 text-af-live'
    default: return 'bg-af-fond text-af-corps'
  }
})

const pointEtatClasses = computed(() => {
  switch (profil.value?.etat) {
    case 'actif': return 'bg-af-vert'
    case 'en_attente': return 'bg-af-chocolat/50 animate-pulse'
    case 'bloque': return 'bg-af-live/50'
    default: return 'bg-af-fond0'
  }
})

const genreLabel = computed(() => {
  const genres: Record<string, string> = {
    homme: 'Homme',
    femme: 'Femme',
    non_precise: 'Non précisé',
  }
  return genres[profil.value?.genre || ''] || profil.value?.genre || 'Non précisé'
})

const langueLabel = computed(() => {
  const langues: Record<string, string> = {
    fr: 'Francais',
    en: 'Anglais',
    pt: 'Portugais',
    ar: 'Arabe',
    sw: 'Swahili',
  }
  return langues[profil.value?.langue_preferee || ''] || profil.value?.langue_preferee || 'Francais'
})

const dateInscription = computed(() => {
  if (!profil.value?.created_at) return ''
  const date = new Date(profil.value.created_at)
  return date.toLocaleDateString('fr-FR', { month: 'short', year: 'numeric' })
})

const dateInscriptionComplete = computed(() => {
  if (!profil.value?.created_at) return ''
  return new Date(profil.value.created_at).toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
})

const dateNaissanceFormatee = computed(() => {
  if (!profil.value?.date_naissance) return null
  return new Date(profil.value.date_naissance).toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
})

const derniereConnexionFormatee = computed(() => {
  if (!profil.value?.derniere_connexion) return 'Jamais'
  return new Date(profil.value.derniere_connexion).toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
})

// ── Methodes ──
const activerEdition = () => {
  if (!profil.value) return
  profilComposable.effacerMessages()
  formulaire.prenom = profil.value.prenom
  formulaire.nom = profil.value.nom
  formulaire.telephone = profil.value.telephone || ''
  formulaire.genre = profil.value.genre
  formulaire.date_naissance = profil.value.date_naissance || null
  formulaire.fonction = profil.value.fonction || ''
  formulaire.biographie = profil.value.biographie || ''
  modeEdition.value = true
}

const annulerEdition = () => {
  profilComposable.effacerMessages()
  modeEdition.value = false
}

const activerEditionLocalisation = () => {
  if (!profil.value) return
  profilComposable.effacerMessages()
  formulaireLocalisation.ville = profil.value.ville || ''
  formulaireLocalisation.localite = profil.value.localite || ''
  formulaireLocalisation.langue_preferee = profil.value.langue_preferee || 'fr'
  modeEditionLocalisation.value = true
}

const sauvegarderProfil = async () => {
  try {
    const resultat = await profilComposable.modifierProfil(formulaire)
    profil.value = resultat
    modeEdition.value = false
  }
  catch {
    // erreur geree par le composable
  }
}

const sauvegarderLocalisation = async () => {
  try {
    const resultat = await profilComposable.modifierProfil(formulaireLocalisation)
    profil.value = resultat
    modeEditionLocalisation.value = false
  }
  catch {
    // erreur geree par le composable
  }
}

const onPhotoChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const fichier = input.files?.[0]
  if (!fichier) return

  try {
    await profilComposable.changerPhoto(fichier)
    // Recharger le profil pour avoir la nouvelle URL
    const p = await profilComposable.chargerProfil()
    profil.value = p
  }
  catch {
    // erreur geree par le composable
  }

  // Reset input
  input.value = ''
}

const handleChangerMotDePasse = async () => {
  try {
    await profilComposable.changerMotDePasse({
      ancien_mot_de_passe: formulaireMdp.ancien_mot_de_passe,
      nouveau_mot_de_passe: formulaireMdp.nouveau_mot_de_passe,
      confirmation_mot_de_passe: formulaireMdp.confirmation_mot_de_passe,
    })
    // Reset formulaire
    formulaireMdp.ancien_mot_de_passe = ''
    formulaireMdp.nouveau_mot_de_passe = ''
    formulaireMdp.confirmation_mot_de_passe = ''
  }
  catch {
    // erreur geree par le composable
  }
}

// ── Init ──
onMounted(async () => {
  if (!isAuthenticated.value) {
    redirigerVersConnexion()
    return
  }

  try {
    profil.value = await profilComposable.chargerProfil()
    // Charger le parcours Africonnect
    try {
      const parcours = await retrouvAmis.listerParcours()
      parcoursRetrouvAmis.value = parcours || []
    } catch {
      // non bloquant
    }
    // Charger le statut de la demande Bibliothèque Humaine
    try {
      chargementDemande.value = true
      maDemande.value = await obtenirMaDemande()
    } catch {
      // non bloquant
    } finally {
      chargementDemande.value = false
    }
    // Charger le statut de la demande d'expertise
    try {
      chargementExpertise.value = true
      maCandidatureExpert.value = await obtenirMaCandidature()
    } catch {
      // non bloquant
    } finally {
      chargementExpertise.value = false
    }
    // Compter les supports médias détenus (chaînes et stations confondues)
    try {
      nombreSupports.value = (await mesSupports()).length
    } catch {
      // non bloquant
    }
  }
  catch {
    // erreur affichee via le composable
  }
  finally {
    chargement.value = false
  }
})
</script>

<style scoped>
@reference "~/assets/css/main.css";

input:focus,
select:focus,
textarea:focus {
  box-shadow: 0 0 0 3px rgba(34, 139, 34, 0.1);
}
</style>
