<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <!-- Élargi depuis `max-w-4xl` : le menu latéral occupe une colonne qui
         n'existait pas quand les sections tenaient dans un menu déroulant. -->
    <div class="max-w-6xl mx-auto px-4">

      <!-- Chargement -->
      <div v-if="chargement" class="flex items-center justify-center py-32">
        <font-awesome-icon icon="fa-solid fa-spinner" class="text-4xl text-custom-chocolat animate-spin" />
      </div>

      <template v-else-if="profil">
        <!-- ═══ Header Profil ═══ -->
        <div
          class="bg-white rounded-2xl shadow-lg overflow-hidden mb-6"
          data-aos="fade-up"
        >
          <!-- Banniere -->
          <div class="h-32 bg-gradient-to-r from-custom-chocolat via-amber-700 to-custom-green relative">
            <div class="absolute inset-0 bg-gradient-to-r from-custom-green/10 to-custom-chocolat/10"></div>
          </div>

          <!-- Avatar + Infos -->
          <div class="px-6 pb-6 -mt-16 relative">
            <div class="flex flex-col sm:flex-row items-center sm:items-end gap-4">
              <!-- Avatar -->
              <div class="relative group">
                <img
                  v-if="profil.photo_url"
                  :src="photoComplete"
                  :alt="nomComplet"
                  class="w-28 h-28 rounded-full object-cover border-4 border-white shadow-lg"
                />
                <div
                  v-else
                  class="w-28 h-28 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-3xl font-bold border-4 border-white shadow-lg"
                >
                  {{ initiaux }}
                </div>
                <!-- Bouton changer photo -->
                <label
                  class="absolute bottom-1 right-1 w-8 h-8 bg-custom-green text-white rounded-full flex items-center justify-center cursor-pointer shadow-md hover:bg-emerald-600 transition-colors opacity-0 group-hover:opacity-100"
                >
                  <font-awesome-icon icon="fa-solid fa-image" class="text-xs" />
                  <input
                    type="file"
                    accept="image/jpeg,image/png,image/webp"
                    class="hidden"
                    @change="onPhotoChange"
                  />
                </label>
              </div>

              <!-- Nom et infos -->
              <div class="text-center sm:text-left flex-1">
                <h1 class="text-2xl font-bold text-gray-800 font-display">{{ nomComplet }}</h1>
                <p class="text-gray-500 text-sm">{{ profil.email }}</p>
                <div class="flex flex-wrap items-center justify-center sm:justify-start gap-2 mt-2">
                  <!-- Badge etat -->
                  <span
                    class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-xs font-medium"
                    :class="badgeEtatClasses"
                  >
                    <span class="w-2 h-2 rounded-full" :class="pointEtatClasses"></span>
                    {{ etatLabel }}
                  </span>
                  <!-- Roles -->
                  <span
                    v-for="role in profil.roles"
                    :key="role"
                    class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-xs font-medium bg-amber-50 text-amber-700"
                  >
                    <font-awesome-icon icon="fa-solid fa-user-tag" class="text-[10px]" />
                    {{ role }}
                  </span>
                  <!-- Date inscription -->
                  <span class="text-xs text-gray-400">
                    <font-awesome-icon icon="fa-solid fa-calendar" class="mr-1" />
                    Membre depuis {{ dateInscription }}
                  </span>
                </div>

                <!-- Accès direct à la gestion des supports détenus : l'onglet
                     existe, mais il siège au dixième rang d'un menu déroulant.
                     Rien ne s'affiche pour qui ne détient aucun support. -->
                <div v-if="nombreSupports > 0" class="mt-3 flex justify-center sm:justify-start">
                  <button
                    type="button"
                    class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-xl border border-custom-chocolat/30 bg-orange-50 text-custom-chocolat hover:bg-orange-100 transition-colors cursor-pointer"
                    @click="allerAuxSupports"
                  >
                    <font-awesome-icon icon="fa-solid fa-tv" class="text-xs" />
                    Gérer mes supports médias
                    <span class="inline-flex items-center justify-center min-w-5 h-5 px-1.5 rounded-full bg-custom-chocolat text-white text-[11px] font-bold">
                      {{ nombreSupports }}
                    </span>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- ═══ Messages succes/erreur ═══ -->
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
            class="mb-4 bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-xl text-sm flex items-center gap-2"
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
            class="mb-4 bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-xl text-sm flex items-center gap-2"
          >
            <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
            {{ profilComposable.error.value }}
          </div>
        </Transition>

        <!-- ═══ Sections du compte ═══
             Menu latéral en écran large, rangée de pilules défilable en mobile.
             Un menu déroulant tenait cette place : dix sections repliées
             derrière un bouton, dont on ne savait l'existence qu'en l'ouvrant. -->
        <div
          ref="zoneOnglets"
          class="grid lg:grid-cols-[264px_minmax(0,1fr)] gap-6 scroll-mt-28"
          data-aos="fade-up"
          data-aos-delay="100"
        >
          <nav aria-label="Sections de mon compte" class="lg:sticky lg:top-28 lg:self-start">
            <!-- Mobile : rangée horizontale, tout est visible d'un coup d'œil
                 ou d'un glissement — les marges négatives laissent les pilules
                 défiler jusqu'aux bords de l'écran. -->
            <div class="lg:hidden -mx-4 px-4 overflow-x-auto">
              <div class="flex w-max gap-2 pb-1">
                <button
                  v-for="tab in onglets"
                  :key="tab.id"
                  type="button"
                  class="inline-flex shrink-0 items-center gap-2 rounded-full px-4 py-2.5 text-sm font-medium whitespace-nowrap transition-colors cursor-pointer"
                  :class="ongletActif === tab.id
                    ? 'bg-custom-chocolat text-white shadow-md'
                    : 'bg-white text-gray-600 shadow-sm hover:text-custom-chocolat'"
                  :aria-current="ongletActif === tab.id ? 'page' : undefined"
                  @click="selectionnerOnglet(tab.id)"
                >
                  <font-awesome-icon :icon="tab.icon" class="text-xs" />
                  {{ tab.label }}
                  <span
                    v-if="tab.id === 'mes-supports' && nombreSupports > 0"
                    class="inline-flex h-5 min-w-5 items-center justify-center rounded-full px-1.5 text-[11px] font-bold"
                    :class="ongletActif === tab.id ? 'bg-white/25 text-white' : 'bg-custom-chocolat text-white'"
                  >
                    {{ nombreSupports }}
                  </span>
                </button>
              </div>
            </div>

            <!-- Écran large : sections nommées, pour que « Mes supports
                 médias » se cherche là où on l'attend. -->
            <div class="hidden lg:block bg-white rounded-2xl shadow-lg p-3 space-y-5">
              <div v-for="groupe in GROUPES_ONGLETS" :key="groupe.titre">
                <p class="px-3 pb-1.5 text-[11px] font-semibold uppercase tracking-wider text-gray-400">
                  {{ groupe.titre }}
                </p>
                <button
                  v-for="tab in groupe.onglets"
                  :key="tab.id"
                  type="button"
                  class="w-full flex items-center gap-3 rounded-xl px-3 py-2.5 text-left text-sm transition-colors cursor-pointer"
                  :class="ongletActif === tab.id
                    ? 'bg-orange-50 text-custom-chocolat font-semibold'
                    : 'text-gray-600 hover:bg-gray-50'"
                  :aria-current="ongletActif === tab.id ? 'page' : undefined"
                  @click="selectionnerOnglet(tab.id)"
                >
                  <font-awesome-icon :icon="tab.icon" class="w-4 text-center" />
                  <span class="min-w-0 truncate">{{ tab.label }}</span>
                  <!-- Combien de supports on détient, lisible sans ouvrir la section -->
                  <span
                    v-if="tab.id === 'mes-supports' && nombreSupports > 0"
                    class="ml-auto inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-custom-chocolat px-1.5 text-[11px] font-bold text-white"
                  >
                    {{ nombreSupports }}
                  </span>
                </button>
              </div>
            </div>
          </nav>

          <!-- Contenu de la section retenue -->
          <div class="bg-white rounded-2xl shadow-lg p-6 min-w-0">

            <!-- ─── Onglet Informations ─── -->
            <div v-if="ongletActif === 'informations'" class="space-y-6">
              <div class="flex items-center justify-between mb-2">
                <h2 class="text-lg font-semibold text-gray-800">Informations personnelles</h2>
                <button
                  v-if="!modeEdition"
                  class="flex items-center gap-2 px-4 py-2 text-sm text-custom-chocolat hover:bg-orange-50 rounded-lg transition-colors"
                  @click="activerEdition"
                >
                  <font-awesome-icon icon="fa-solid fa-pen-to-square" />
                  Modifier
                </button>
              </div>

              <!-- Mode Lecture -->
              <div v-if="!modeEdition" class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ProfilChampLecture label="Prenom" :valeur="profil.prenom" icon="fa-solid fa-user" />
                <ProfilChampLecture label="Nom" :valeur="profil.nom" icon="fa-solid fa-user" />
                <ProfilChampLecture label="Email" :valeur="profil.email" icon="fa-solid fa-envelope" />
                <ProfilChampLecture label="Telephone" :valeur="profil.telephone" icon="fa-solid fa-phone" />
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
                    <label class="text-sm font-medium text-gray-700 block">Prenom</label>
                    <input
                      v-model="formulaire.prenom"
                      type="text"
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Nom</label>
                    <input
                      v-model="formulaire.nom"
                      type="text"
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Telephone</label>
                    <input
                      v-model="formulaire.telephone"
                      type="tel"
                      placeholder="+225 00 00 00 00"
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Genre</label>
                    <select
                      v-model="formulaire.genre"
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    >
                      <option value="non_precise">Non precise</option>
                      <option value="homme">Homme</option>
                      <option value="femme">Femme</option>
                    </select>
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Date de naissance</label>
                    <input
                      v-model="formulaire.date_naissance"
                      type="date"
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Fonction</label>
                    <input
                      v-model="formulaire.fonction"
                      type="text"
                      placeholder="Ex: Developpeur, Enseignant..."
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                </div>
                <div class="space-y-2">
                  <label class="text-sm font-medium text-gray-700 block">Biographie</label>
                  <textarea
                    v-model="formulaire.biographie"
                    rows="4"
                    placeholder="Parlez de vous..."
                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white resize-none"
                  ></textarea>
                </div>

                <!-- Boutons actions -->
                <div class="flex items-center justify-end gap-3 pt-2">
                  <button
                    type="button"
                    class="px-5 py-2.5 text-sm text-gray-600 hover:bg-gray-100 rounded-xl transition-colors"
                    @click="annulerEdition"
                  >
                    Annuler
                  </button>
                  <button
                    type="submit"
                    class="flex items-center gap-2 px-5 py-2.5 text-sm bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300 disabled:opacity-50"
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
                <h2 class="text-lg font-semibold text-gray-800">Localisation et preferences</h2>
                <button
                  v-if="!modeEditionLocalisation"
                  class="flex items-center gap-2 px-4 py-2 text-sm text-custom-chocolat hover:bg-orange-50 rounded-lg transition-colors"
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
                    <label class="text-sm font-medium text-gray-700 block">Ville</label>
                    <input
                      v-model="formulaireLocalisation.ville"
                      type="text"
                      placeholder="Ex: Abidjan, Dakar..."
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Localite / Quartier</label>
                    <input
                      v-model="formulaireLocalisation.localite"
                      type="text"
                      placeholder="Ex: Cocody, Plateau..."
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Langue preferee</label>
                    <select
                      v-model="formulaireLocalisation.langue_preferee"
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
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
                    class="px-5 py-2.5 text-sm text-gray-600 hover:bg-gray-100 rounded-xl transition-colors"
                    @click="modeEditionLocalisation = false"
                  >
                    Annuler
                  </button>
                  <button
                    type="submit"
                    class="flex items-center gap-2 px-5 py-2.5 text-sm bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300 disabled:opacity-50"
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

            <!-- ─── Onglet Retrouve Amis ─── -->
            <div v-if="ongletActif === 'retrouve-amis'" class="space-y-6">
              <h2 class="text-lg font-semibold text-gray-800 mb-2">Retrouve Amis</h2>

              <!-- Statut trouvable -->
              <div class="bg-gray-50 rounded-xl p-5">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div
                      class="w-9 h-9 rounded-lg flex items-center justify-center"
                      :class="profil.est_trouvable ? 'bg-custom-green/10 text-custom-green' : 'bg-gray-200 text-gray-400'"
                    >
                      <font-awesome-icon icon="fa-solid fa-eye" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-sm font-medium text-gray-800">Profil trouvable</p>
                      <p class="text-xs text-gray-500">
                        {{ profil.est_trouvable ? 'Votre profil est visible pour le matching' : 'Votre profil est masque' }}
                      </p>
                    </div>
                  </div>
                  <span
                    class="inline-flex items-center gap-1 px-3 py-1 rounded-full text-xs font-medium"
                    :class="profil.est_trouvable ? 'bg-custom-green/10 text-custom-green' : 'bg-gray-100 text-gray-500'"
                  >
                    {{ profil.est_trouvable ? 'Actif' : 'Inactif' }}
                  </span>
                </div>
              </div>

              <!-- Parcours -->
              <div class="border border-gray-200 rounded-xl p-5">
                <div class="flex items-center gap-3 mb-4">
                  <div class="w-9 h-9 rounded-lg bg-custom-chocolat/10 text-custom-chocolat flex items-center justify-center">
                    <font-awesome-icon icon="fa-solid fa-route" class="text-sm" />
                  </div>
                  <div>
                    <h3 class="text-sm font-semibold text-gray-800">Mon parcours</h3>
                    <p class="text-xs text-gray-500">{{ parcoursRetrouvAmis.length }} entree(s) de parcours</p>
                  </div>
                </div>

                <!-- Liste parcours -->
                <div v-if="parcoursRetrouvAmis.length > 0" class="space-y-2 mb-4">
                  <div
                    v-for="p in parcoursRetrouvAmis"
                    :key="p.id"
                    class="flex items-center gap-3 px-3 py-2 bg-gray-50 rounded-lg text-sm"
                  >
                    <font-awesome-icon
                      :icon="p.type_entree === 'ecole' ? 'fa-solid fa-graduation-cap' : p.type_entree === 'ville_residence' ? 'fa-solid fa-building' : 'fa-solid fa-briefcase'"
                      class="text-gray-400 w-4"
                    />
                    <span class="font-medium text-gray-700">{{ p.nom }}</span>
                    <span v-if="p.ville" class="text-gray-400">- {{ p.ville }}</span>
                    <span v-if="p.periode_debut" class="text-gray-400 ml-auto text-xs">
                      {{ p.periode_debut }}{{ p.periode_fin ? ` - ${p.periode_fin}` : '' }}
                    </span>
                  </div>
                </div>

                <NuxtLink
                  to="/retrouve-amis"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm text-custom-chocolat hover:bg-orange-50 rounded-lg transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-arrow-right" class="text-xs" />
                  Gerer dans Retrouve Amis
                </NuxtLink>
              </div>
            </div>

            <!-- ─── Onglet Securite ─── -->
            <div v-if="ongletActif === 'securite'" class="space-y-6">
              <h2 class="text-lg font-semibold text-gray-800 mb-2">Securite du compte</h2>

              <!-- Infos compte -->
              <div class="bg-gray-50 rounded-xl p-5 space-y-4">
                <h3 class="text-sm font-semibold text-gray-700 uppercase tracking-wide">Informations du compte</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-emerald-100 text-emerald-600 flex items-center justify-center">
                      <font-awesome-icon icon="fa-solid fa-envelope" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-gray-500">Email verifie</p>
                      <p class="text-sm font-medium" :class="profil.email_verifie ? 'text-emerald-600' : 'text-red-500'">
                        {{ profil.email_verifie ? 'Oui' : 'Non' }}
                      </p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-blue-100 text-blue-600 flex items-center justify-center">
                      <font-awesome-icon icon="fa-solid fa-calendar" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-gray-500">Date d'inscription</p>
                      <p class="text-sm font-medium text-gray-700">{{ dateInscriptionComplete }}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-purple-100 text-purple-600 flex items-center justify-center">
                      <font-awesome-icon icon="fa-solid fa-clock-rotate-left" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-gray-500">Derniere connexion</p>
                      <p class="text-sm font-medium text-gray-700">{{ derniereConnexionFormatee }}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg flex items-center justify-center"
                      :class="profil.etat === 'actif' ? 'bg-emerald-100 text-emerald-600' : 'bg-orange-100 text-orange-600'"
                    >
                      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-sm" />
                    </div>
                    <div>
                      <p class="text-xs text-gray-500">Etat du compte</p>
                      <p class="text-sm font-medium" :class="profil.etat === 'actif' ? 'text-emerald-600' : 'text-orange-600'">
                        {{ etatLabel }}
                      </p>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Changer mot de passe -->
              <div class="border border-gray-200 rounded-xl p-5">
                <div class="flex items-center gap-3 mb-4">
                  <div class="w-9 h-9 rounded-lg bg-red-100 text-red-600 flex items-center justify-center">
                    <font-awesome-icon icon="fa-solid fa-lock" class="text-sm" />
                  </div>
                  <h3 class="text-sm font-semibold text-gray-800">Changer le mot de passe</h3>
                </div>

                <form @submit.prevent="handleChangerMotDePasse" class="space-y-4">
                  <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700 block">Mot de passe actuel</label>
                    <input
                      v-model="formulaireMdp.ancien_mot_de_passe"
                      type="password"
                      required
                      class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                    />
                  </div>
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="space-y-2">
                      <label class="text-sm font-medium text-gray-700 block">Nouveau mot de passe</label>
                      <input
                        v-model="formulaireMdp.nouveau_mot_de_passe"
                        type="password"
                        required
                        minlength="6"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                      />
                    </div>
                    <div class="space-y-2">
                      <label class="text-sm font-medium text-gray-700 block">Confirmer le mot de passe</label>
                      <input
                        v-model="formulaireMdp.confirmation_mot_de_passe"
                        type="password"
                        required
                        minlength="6"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                      />
                    </div>
                  </div>
                  <div class="flex justify-end">
                    <button
                      type="submit"
                      class="flex items-center gap-2 px-5 py-2.5 text-sm bg-red-600 text-white font-medium rounded-xl hover:bg-red-700 transition-colors disabled:opacity-50"
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
              <h2 class="text-lg font-semibold text-gray-800">Bibliothèque Humaine</h2>

              <div v-if="chargementDemande" class="flex justify-center py-8">
                <font-awesome-icon icon="fa-solid fa-spinner" class="text-2xl text-custom-chocolat animate-spin" />
              </div>

              <div v-else-if="!maDemande" class="text-center py-10">
                <font-awesome-icon icon="fa-solid fa-book-open" class="text-4xl text-gray-300 mb-3" />
                <p class="text-gray-500 text-sm mb-4">Vous n'avez pas encore soumis de demande pour rejoindre la Bibliothèque Humaine.</p>
                <NuxtLink
                  to="/bibliotheque/humaine"
                  class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-linear-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  Soumettre une demande
                </NuxtLink>
              </div>

              <div v-else class="space-y-4">
                <!-- Badge statut -->
                <div
                  class="flex items-center gap-3 p-4 rounded-xl border"
                  :class="{
                    'bg-amber-50 border-amber-200': maDemande.statut === 'en_attente',
                    'bg-green-50 border-green-200': maDemande.statut === 'valide',
                    'bg-red-50 border-red-200': maDemande.statut === 'rejete',
                  }"
                >
                  <font-awesome-icon
                    :icon="maDemande.statut === 'valide' ? 'fa-solid fa-circle-check' : maDemande.statut === 'rejete' ? 'fa-solid fa-circle-xmark' : 'fa-solid fa-clock'"
                    class="text-xl"
                    :class="{
                      'text-amber-500': maDemande.statut === 'en_attente',
                      'text-green-600': maDemande.statut === 'valide',
                      'text-red-600': maDemande.statut === 'rejete',
                    }"
                  />
                  <div class="flex-1">
                    <p
                      class="font-semibold text-sm"
                      :class="{
                        'text-amber-700': maDemande.statut === 'en_attente',
                        'text-green-700': maDemande.statut === 'valide',
                        'text-red-700': maDemande.statut === 'rejete',
                      }"
                    >
                      {{
                        maDemande.statut === 'en_attente' ? 'En attente de validation'
                        : maDemande.statut === 'valide' ? 'Demande validée'
                        : 'Demande rejetée'
                      }}
                    </p>
                    <p class="text-xs text-gray-500 mt-0.5">
                      Soumis le {{ new Date(maDemande.createdAt).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) }}
                    </p>
                  </div>
                </div>

                <!-- Commentaire admin si rejeté -->
                <div v-if="maDemande.statut === 'rejete' && maDemande.commentaireAdmin" class="p-4 bg-red-50 border border-red-200 rounded-xl">
                  <p class="text-xs font-semibold text-red-600 uppercase tracking-wide mb-1">Motif du rejet</p>
                  <p class="text-sm text-red-700">{{ maDemande.commentaireAdmin }}</p>
                </div>

                <!-- Re-soumettre si rejeté -->
                <div v-if="maDemande.statut === 'rejete'" class="text-center pt-2">
                  <NuxtLink
                    to="/bibliotheque/humaine"
                    class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-linear-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300"
                  >
                    <font-awesome-icon icon="fa-solid fa-rotate-right" />
                    Soumettre une nouvelle demande
                  </NuxtLink>
                </div>

                <!-- Résumé -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                  <div>
                    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Fonction déclarée</p>
                    <p class="text-sm text-gray-700">{{ maDemande.fonction }}</p>
                  </div>
                  <div v-if="maDemande.pays">
                    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Territoire</p>
                    <p class="text-sm text-gray-700">{{ maDemande.pays }}</p>
                  </div>
                  <div v-if="maDemande.specialites.length > 0" class="md:col-span-2">
                    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Spécialités</p>
                    <div class="flex flex-wrap gap-1">
                      <span
                        v-for="s in maDemande.specialites"
                        :key="s"
                        class="inline-block px-2 py-0.5 bg-gray-100 text-gray-600 text-xs rounded-full"
                      >{{ s }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- ─── Onglet Expertise ─── -->
            <div v-if="ongletActif === 'expertise'" class="space-y-6">
              <h2 class="text-lg font-semibold text-gray-800">Mon expertise</h2>

              <div v-if="chargementExpertise" class="flex justify-center py-8">
                <font-awesome-icon icon="fa-solid fa-spinner" class="text-2xl text-custom-chocolat animate-spin" />
              </div>

              <div v-else-if="!maCandidatureExpert" class="text-center py-10">
                <font-awesome-icon icon="fa-solid fa-user-tie" class="text-4xl text-gray-300 mb-3" />
                <p class="text-gray-500 text-sm mb-4">Vous n'avez pas encore soumis de demande pour devenir expert.</p>
                <NuxtLink
                  to="/devenir-expert"
                  class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-linear-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  Apporter mon expertise
                </NuxtLink>
              </div>

              <div v-else class="space-y-4">
                <!-- Badge statut -->
                <div
                  class="flex items-center gap-3 p-4 rounded-xl border"
                  :class="{
                    'bg-amber-50 border-amber-200': maCandidatureExpert.statut === 'en_attente',
                    'bg-green-50 border-green-200': maCandidatureExpert.statut === 'valide',
                    'bg-red-50 border-red-200': maCandidatureExpert.statut === 'refuse',
                  }"
                >
                  <font-awesome-icon
                    :icon="maCandidatureExpert.statut === 'valide' ? 'fa-solid fa-circle-check' : maCandidatureExpert.statut === 'refuse' ? 'fa-solid fa-circle-xmark' : 'fa-solid fa-clock'"
                    class="text-xl"
                    :class="{
                      'text-amber-500': maCandidatureExpert.statut === 'en_attente',
                      'text-green-600': maCandidatureExpert.statut === 'valide',
                      'text-red-600': maCandidatureExpert.statut === 'refuse',
                    }"
                  />
                  <div class="flex-1">
                    <p
                      class="font-semibold text-sm"
                      :class="{
                        'text-amber-700': maCandidatureExpert.statut === 'en_attente',
                        'text-green-700': maCandidatureExpert.statut === 'valide',
                        'text-red-700': maCandidatureExpert.statut === 'refuse',
                      }"
                    >
                      {{
                        maCandidatureExpert.statut === 'en_attente' ? 'En attente de validation'
                        : maCandidatureExpert.statut === 'valide' ? 'Demande validée'
                        : 'Demande refusée'
                      }}
                    </p>
                    <p class="text-xs text-gray-500 mt-0.5">
                      Soumis le {{ new Date(maCandidatureExpert.createdAt).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) }}
                    </p>
                  </div>
                </div>

                <!-- Commentaire admin si refusé -->
                <div v-if="maCandidatureExpert.statut === 'refuse' && maCandidatureExpert.commentaireAdmin" class="p-4 bg-red-50 border border-red-200 rounded-xl">
                  <p class="text-xs font-semibold text-red-600 uppercase tracking-wide mb-1">Motif du refus</p>
                  <p class="text-sm text-red-700">{{ maCandidatureExpert.commentaireAdmin }}</p>
                </div>

                <!-- Re-soumettre si refusé -->
                <div v-if="maCandidatureExpert.statut === 'refuse'" class="text-center pt-2">
                  <NuxtLink
                    to="/devenir-expert"
                    class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-linear-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300"
                  >
                    <font-awesome-icon icon="fa-solid fa-rotate-left" />
                    Soumettre une nouvelle demande
                  </NuxtLink>
                </div>

                <!-- Lien fiche publique si validé -->
                <div v-if="maCandidatureExpert.statut === 'valide' && profil" class="text-center pt-2">
                  <NuxtLink
                    :to="`/experts/${profil.id}`"
                    class="inline-flex items-center gap-2 px-5 py-2.5 text-sm border border-custom-green text-custom-green font-medium rounded-xl hover:bg-green-50 transition-all duration-300"
                  >
                    <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" />
                    Voir ma fiche d'expert
                  </NuxtLink>
                </div>

                <!-- Résumé -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                  <div>
                    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Domaine</p>
                    <p class="text-sm text-gray-700">{{ maCandidatureExpert.domaine }}</p>
                  </div>
                  <div>
                    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Années d'expérience</p>
                    <p class="text-sm text-gray-700">{{ maCandidatureExpert.nbAnneesExperience }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- ─── Onglet Mes échanges sabbatiques ─── -->
            <div v-if="ongletActif === 'mes-echanges'" class="space-y-6">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-gray-800">Mes projets d'échange</h2>
                <NuxtLink
                  to="/echanges-sabbatiques/proposer"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-custom-green text-white font-medium rounded-xl hover:bg-custom-green/90 transition-all"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  <span class="hidden sm:inline">Proposer</span>
                </NuxtLink>
              </div>
              <p class="text-sm text-gray-500 -mt-3">
                Gérez les candidatures reçues et sélectionnez le candidat final pour chaque projet.
              </p>
              <SabbatiqueMesEchanges />
            </div>

            <!-- ─── Onglet Mes événements ─── -->
            <div v-if="ongletActif === 'mes-evenements'" class="space-y-6">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-gray-800">Mes événements</h2>
                <NuxtLink
                  to="/evenements/liste"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-custom-green text-white font-medium rounded-xl hover:bg-custom-green/90 transition-all"
                >
                  <font-awesome-icon icon="fa-solid fa-plus" />
                  <span class="hidden sm:inline">Proposer</span>
                </NuxtLink>
              </div>
              <p class="text-sm text-gray-500 -mt-3">
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
                <h2 class="text-lg font-semibold text-gray-800">Mes supports médias</h2>
                <NuxtLink
                  to="/mon-compte/invitations-medias"
                  class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-custom-green text-white font-medium rounded-xl hover:bg-custom-green/90 transition-all"
                >
                  <font-awesome-icon icon="fa-solid fa-envelope" />
                  <span class="hidden sm:inline">Invitations</span>
                </NuxtLink>
              </div>
              <p class="text-sm text-gray-500 -mt-3">
                Les chaînes et stations que vous détenez : bâtissez leur grille de
                programmation, arbitrez les idées et demandes d'animation reçues,
                et gérez votre équipe.
              </p>
              <MediaMesSupports />
            </div>

          </div>
        </div>
      </template>

    </div>
  </div>
</template>

<script setup lang="ts">
import type { Profil, ModifierProfilForm } from '~/composables/useProfil'
import type { MaDemandeAPI } from '~/composables/useBibliothequeHumaine'
import type { MaCandidatureAPI } from '~/composables/useExperts'

useHead({
  title: 'Mon profil - AfricanS',
})

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

// Retrouve Amis
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
// regroupement dit aussi *pourquoi* on chercherait chaque section — réglages du
// compte, statut d'une candidature, ou ce qu'on anime sur la plateforme.
interface OngletProfil { id: string, label: string, icon: string }

const GROUPES_ONGLETS: { titre: string, onglets: OngletProfil[] }[] = [
  {
    titre: 'Mon compte',
    onglets: [
      { id: 'informations', label: 'Informations', icon: 'fa-solid fa-user' },
      { id: 'localisation', label: 'Localisation', icon: 'fa-solid fa-location-dot' },
      { id: 'securite', label: 'Securite', icon: 'fa-solid fa-lock' },
    ],
  },
  {
    titre: 'Mes demandes',
    onglets: [
      { id: 'bibliotheque-humaine', label: 'Bibliothèque', icon: 'fa-solid fa-book-open' },
      { id: 'expertise', label: 'Expertise', icon: 'fa-solid fa-user-tie' },
      { id: 'retrouve-amis', label: 'Retrouve Amis', icon: 'fa-solid fa-users' },
    ],
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

const initiaux = computed(() => {
  if (!profil.value) return ''
  return `${profil.value.prenom?.charAt(0)?.toUpperCase() || ''}${profil.value.nom?.charAt(0)?.toUpperCase() || ''}`
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
    case 'actif': return 'bg-emerald-50 text-emerald-700'
    case 'en_attente': return 'bg-orange-50 text-orange-700'
    case 'bloque': return 'bg-red-50 text-red-700'
    default: return 'bg-gray-50 text-gray-700'
  }
})

const pointEtatClasses = computed(() => {
  switch (profil.value?.etat) {
    case 'actif': return 'bg-emerald-500'
    case 'en_attente': return 'bg-orange-500 animate-pulse'
    case 'bloque': return 'bg-red-500'
    default: return 'bg-gray-500'
  }
})

const genreLabel = computed(() => {
  const genres: Record<string, string> = {
    homme: 'Homme',
    femme: 'Femme',
    non_precise: 'Non precise',
  }
  return genres[profil.value?.genre || ''] || profil.value?.genre || 'Non precise'
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
    // Charger le parcours Retrouve Amis
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
