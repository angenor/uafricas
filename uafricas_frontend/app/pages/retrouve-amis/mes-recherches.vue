<script setup lang="ts">
definePageMeta({ layout: false })

const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { listerAvis, cloturerAvis, supprimerAvis } = useRetrouvAmis()

const avisListe = ref<any[]>([])
const chargement = ref(false)
const filtreEtat = ref('tous')
const page = ref(1)
const parPage = ref(9)
const total = ref(0)

const optionsEtat = [
  { valeur: 'tous', libelle: 'Tous les etats' },
  { valeur: 'actif', libelle: 'Actifs' },
  { valeur: 'cloture', libelle: 'Clotures' },
  { valeur: 'suspendu', libelle: 'Suspendus' },
]

const totalPages = computed(() => Math.ceil(total.value / parPage.value))

const chargerAvis = async () => {
  chargement.value = true
  try {
    const params: Record<string, any> = {
      page: page.value,
      par_page: parPage.value,
    }
    if (filtreEtat.value !== 'tous') {
      params.etat = filtreEtat.value
    }
    const res = await listerAvis(params)
    avisListe.value = res.avis ?? []
    total.value = res.total ?? 0
  }
  catch {
    avisListe.value = []
  }
  finally {
    chargement.value = false
  }
}

const onFiltrerEtat = (etat: string) => {
  filtreEtat.value = etat
  page.value = 1
  chargerAvis()
}

const onChangerPage = (nouvellePage: number) => {
  page.value = nouvellePage
  chargerAvis()
}

const onVoir = (slug: string) => {
  navigateTo(`/retrouve-amis/public/${slug}`)
}

const onModifier = (id: string) => {
  navigateTo(`/retrouve-amis/nouveau?id=${id}`)
}

// ── Cloture ─────────────────────────────────────────────────
const confirmationCloture = ref<string | null>(null)

const onDemanderCloture = (id: string) => {
  confirmationCloture.value = id
}

const onAnnulerCloture = () => {
  confirmationCloture.value = null
}

const onConfirmerCloture = async () => {
  if (!confirmationCloture.value) return
  try {
    await cloturerAvis(confirmationCloture.value)
    confirmationCloture.value = null
    chargerAvis()
  }
  catch {
    // erreur geree par le composable
  }
}

// ── Suppression ─────────────────────────────────────────────
const confirmationSuppression = ref<string | null>(null)

const onDemanderSuppression = (id: string) => {
  confirmationSuppression.value = id
}

const onAnnulerSuppression = () => {
  confirmationSuppression.value = null
}

const onConfirmerSuppression = async () => {
  if (!confirmationSuppression.value) return
  try {
    await supprimerAvis(confirmationSuppression.value)
    confirmationSuppression.value = null
    chargerAvis()
  }
  catch {
    // erreur geree par le composable
  }
}

const badgeCouleur = (etat: string): string => {
  switch (etat) {
    case 'actif': return 'bg-af-vert/10 text-af-vert'
    case 'cloture': return 'bg-af-fond text-af-corps'
    case 'suspendu': return 'bg-af-chocolat/10 text-af-chocolat'
    default: return 'bg-af-fond text-af-corps'
  }
}

const labelRelation = (type: string | null, autre: string | null): string | null => {
  if (!type) return null
  const map: Record<string, string> = {
    amis_enfance: 'Amis d\'enfance',
    amis_ecole: 'Amis d\'ecole',
    collegue: 'Collegue',
    connaissance: 'Connaissance',
    frere_soeur: 'Frere / Soeur',
    parent: 'Parent',
    autre: autre ? `Autre : ${autre}` : 'Autre',
  }
  return map[type] ?? type
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  chargerAvis()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était hotlinkée sur Unsplash ; le hero local du module
           existait déjà. -->
      <AfricansBandeauModule
        titre="Mes recherches"
        sous-titre="Suivez l'état de vos avis de recherche et gérez-les facilement."
        image="/images/africans/heros/hero-africonnect.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/retrouve-amis' },
          { libelle: 'Africonnect', vers: '/retrouve-amis' },
          { libelle: 'Mes recherches' },
        ]"
      >
        <template #action>
          <AfricansBouton vers="/retrouve-amis/nouveau" icone="fa-solid fa-plus">
            Nouvel avis
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="min-w-0">
        <!-- Barre de filtres -->
        <div class="bg-white rounded-lg shadow-sm border border-af-bordure p-3 mb-6">
          <div class="flex items-center gap-3">
            <label for="filtre-etat" class="text-sm font-medium text-af-corps shrink-0">
              Etat :
            </label>
            <select
              id="filtre-etat"
              :value="filtreEtat"
              class="block w-full sm:w-56 px-3 py-2 bg-white border border-af-bordure rounded-lg text-sm text-af-corps focus:outline-none focus:ring-2 focus:border-af-chocolat focus:border-af-chocolat"
              @change="onFiltrerEtat(($event.target as HTMLSelectElement).value)"
            >
              <option v-for="opt in optionsEtat" :key="opt.valeur" :value="opt.valeur">
                {{ opt.libelle }}
              </option>
            </select>
          </div>
        </div>

        <!-- Chargement -->
        <div v-if="chargement" class="flex items-center justify-center py-20">
          <font-awesome-icon :icon="['fas', 'spinner']" class="text-3xl text-af-chocolat animate-spin" />
        </div>

        <!-- Etat vide -->
        <div v-else-if="avisListe.length === 0" class="bg-white rounded-lg shadow-sm border border-af-bordure p-12 text-center">
          <div class="w-20 h-20 mx-auto mb-6 bg-af-fond text-af-atone-2 rounded-full flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'users-slash']" class="text-3xl" />
          </div>
          <h3 class="text-xl font-semibold text-af-corps mb-2">
            Vous n'avez pas encore d'avis de recherche
          </h3>
          <p class="text-af-atone mb-6">
            Commencez par deposer un avis pour retrouver un proche perdu de vue.
          </p>
          <NuxtLink
            to="/retrouve-amis/nouveau"
            class="inline-flex items-center gap-2 px-6 py-3 bg-af-chocolat text-white font-semibold rounded-lg hover:opacity-90 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'plus']" />
            Creer un avis de recherche
          </NuxtLink>
        </div>

        <!-- Grille des avis -->
        <div v-else>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div
              v-for="avis in avisListe"
              :key="avis.id"
              class="bg-white rounded-lg shadow-sm border border-af-bordure overflow-hidden hover:shadow-md transition-shadow"
            >
              <!-- Photo ou placeholder -->
              <div class="h-40 bg-af-fond overflow-hidden">
                <img
                  v-if="avis.photo_url"
                  :src="avis.photo_url"
                  :alt="avis.nom_recherche"
                  class="w-full h-full object-cover"
                >
                <div v-else class="w-full h-full flex flex-col items-center justify-center text-af-atone-2">
                  <font-awesome-icon :icon="['fas', 'user']" class="text-5xl mb-2" />
                  <span class="text-xs text-af-atone-2">Photo non disponible</span>
                </div>
              </div>

              <div class="p-5">
                <div class="flex items-start justify-between mb-3">
                  <h3 class="text-lg font-semibold text-af-encre line-clamp-1">
                    {{ avis.nom_recherche }} {{ avis.prenom_recherche }}
                  </h3>
                  <span
                    class="shrink-0 ml-2 px-2.5 py-0.5 text-xs font-medium rounded-full capitalize"
                    :class="badgeCouleur(avis.etat)"
                  >
                    {{ avis.etat }}
                  </span>
                </div>

                <!-- Type de relation -->
                <p v-if="avis.type_relation" class="text-sm text-af-atone mb-1">
                  <font-awesome-icon :icon="['fas', 'heart']" class="mr-1 text-af-chocolat" />
                  {{ labelRelation(avis.type_relation, avis.type_relation_autre) }}
                </p>

                <!-- Genre -->
                <p v-if="avis.genre_recherche" class="text-sm text-af-atone mb-1">
                  <font-awesome-icon :icon="['fas', 'user']" class="mr-1" />
                  {{ avis.genre_recherche === 'homme' ? 'Homme' : 'Femme' }}
                </p>

                <!-- Lieu -->
                <p v-if="avis.ville_rencontre || avis.localite_rencontre" class="text-sm text-af-atone mb-1">
                  <font-awesome-icon :icon="['fas', 'location-dot']" class="mr-1" />
                  {{ avis.localite_rencontre }}{{ avis.localite_rencontre && avis.ville_rencontre ? ', ' : '' }}{{ avis.ville_rencontre }}
                </p>

                <!-- Reseaux sociaux -->
                <p v-if="avis.rencontre_reseaux_sociaux && avis.reseaux_sociaux" class="text-sm text-af-atone mb-1">
                  <font-awesome-icon :icon="['fas', 'share-nodes']" class="mr-1" />
                  {{ avis.reseaux_sociaux.split(',').join(', ') }}
                </p>

                <!-- Description -->
                <p v-if="avis.description_physique" class="text-sm text-af-corps line-clamp-2 mt-2 mb-3">
                  {{ avis.description_physique }}
                </p>

                <!-- Lien public -->
                <div v-if="avis.est_public && avis.slug && avis.etat === 'actif'" class="mb-3 p-2.5 bg-af-vert/5 rounded-lg">
                  <NuxtLink
                    :to="`/retrouve-amis/public/${avis.slug}`"
                    class="text-xs text-af-vert hover:opacity-80 hover:underline break-all"
                    target="_blank"
                  >
                    <font-awesome-icon :icon="['fas', 'globe']" class="mr-1" />
                    Avis public
                    <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="ml-1" />
                  </NuxtLink>
                </div>

                <!-- Date -->
                <p class="text-xs text-af-atone-2 mb-3">
                  <font-awesome-icon :icon="['fas', 'clock']" class="mr-1" />
                  {{ new Date(avis.created_at).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' }) }}
                </p>

                <!-- Boutons d'action -->
                <div class="flex items-center gap-1.5 pt-3 border-t border-af-bordure">
                  <button
                    class="flex-1 px-3 py-2 text-xs font-medium text-af-chocolat bg-af-chocolat/5 rounded-lg hover:bg-af-chocolat/10 transition-colors cursor-pointer text-center"
                    title="Voir l'avis public"
                    @click="onVoir(avis.slug)"
                  >
                    <font-awesome-icon :icon="['fas', 'eye']" class="mr-1" />
                    Voir
                  </button>
                  <button
                    v-if="avis.etat === 'actif'"
                    class="flex-1 px-3 py-2 text-xs font-medium text-af-chocolat bg-af-chocolat/5 rounded-lg hover:bg-af-chocolat/10 transition-colors cursor-pointer text-center"
                    title="Modifier l'avis"
                    @click="onModifier(avis.id)"
                  >
                    <font-awesome-icon :icon="['fas', 'pen']" class="mr-1" />
                    Modifier
                  </button>
                  <button
                    v-if="avis.etat === 'actif'"
                    class="flex-1 px-3 py-2 text-xs font-medium text-af-chocolat bg-af-chocolat/5 rounded-lg hover:bg-af-chocolat/10 transition-colors cursor-pointer text-center"
                    title="Cloturer l'avis"
                    @click="onDemanderCloture(avis.id)"
                  >
                    <font-awesome-icon :icon="['fas', 'circle-xmark']" class="mr-1" />
                    Cloturer
                  </button>
                  <button
                    class="px-2.5 py-2 text-xs font-medium text-af-live bg-af-live/5 rounded-lg hover:bg-af-live/10 transition-colors cursor-pointer"
                    title="Supprimer definitivement"
                    @click="onDemanderSuppression(avis.id)"
                  >
                    <font-awesome-icon :icon="['fas', 'trash']" />
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Pagination -->
          <div v-if="totalPages > 1" class="flex items-center justify-center gap-2 mt-10">
            <button
              class="px-3 py-2 text-sm font-medium rounded-lg transition-colors cursor-pointer"
              :class="page === 1 ? 'text-af-atone-2 cursor-not-allowed' : 'text-af-corps hover:bg-af-fond'"
              :disabled="page === 1"
              @click="onChangerPage(page - 1)"
            >
              <font-awesome-icon :icon="['fas', 'chevron-left']" />
            </button>
            <template v-for="p in totalPages" :key="p">
              <button
                class="w-10 h-10 text-sm font-medium rounded-lg transition-colors cursor-pointer"
                :class="p === page ? 'bg-af-chocolat text-white' : 'text-af-corps hover:bg-af-fond'"
                @click="onChangerPage(p)"
              >
                {{ p }}
              </button>
            </template>
            <button
              class="px-3 py-2 text-sm font-medium rounded-lg transition-colors cursor-pointer"
              :class="page === totalPages ? 'text-af-atone-2 cursor-not-allowed' : 'text-af-corps hover:bg-af-fond'"
              :disabled="page === totalPages"
              @click="onChangerPage(page + 1)"
            >
              <font-awesome-icon :icon="['fas', 'chevron-right']" />
            </button>
          </div>
        </div>
    </div>

    <AfricansModale
      :model-value="confirmationCloture"
      titre="Confirmer la clôture"
      icone="fa-solid fa-circle-xmark"
      ton="chocolat"
      @update:model-value="onAnnulerCloture"
    >
      <p class="text-[14px]/[1.6] text-af-corps">
        Cet avis ne sera plus proposé aux visiteurs ni rapproché de nouveaux profils.
        Il reste consultable depuis cette page.
      </p>

      <template #actions>
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70"
          @click="onAnnulerCloture"
        >
          Annuler
        </button>
        <AfricansBouton icone="fa-solid fa-circle-xmark" @click="onConfirmerCloture">
          Oui, clôturer
        </AfricansBouton>
      </template>
    </AfricansModale>

    <AfricansModale
      :model-value="confirmationSuppression"
      titre="Supprimer cet avis"
      icone="fa-solid fa-triangle-exclamation"
      ton="chocolat"
      @update:model-value="onAnnulerSuppression"
    >
      <p class="text-[14px]/[1.6] text-af-corps">
        Êtes-vous sûr de vouloir supprimer définitivement cet avis de recherche ?
        <strong class="font-bold text-af-live">Cette action est irréversible.</strong>
      </p>

      <template #actions>
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70"
          @click="onAnnulerSuppression"
        >
          Annuler
        </button>
        <!-- Bouton brut : la variante destructrice n'existe pas parmi les
             trois variantes d'AfricansBouton, et la couleur porte le sens. -->
        <button
          type="button"
          class="inline-flex h-10 items-center justify-center gap-2 rounded-lg bg-af-live px-6 text-base font-bold text-white transition hover:opacity-90"
          @click="onConfirmerSuppression"
        >
          <font-awesome-icon icon="fa-solid fa-trash" />
          Oui, supprimer
        </button>
      </template>
    </AfricansModale>

    <template #rail>
      <RetrouveAmisSideBar />
    </template>
  </NuxtLayout>
</template>
