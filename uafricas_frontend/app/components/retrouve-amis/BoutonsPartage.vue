<script setup lang="ts">
const props = defineProps<{
  slug: string
  compteurPartages: number
  nomRecherche: string
  prenomRecherche?: string
}>()

const { incrementerPartage } = useRetrouvAmis()
const compteur = ref(props.compteurPartages)
const copieLienOk = ref(false)

const urlPage = computed(() => `https://www.africans-world.org/retrouve-amis/public/${props.slug}`)
const nomComplet = computed(() => {
  return props.prenomRecherche
    ? `${props.nomRecherche} ${props.prenomRecherche}`
    : props.nomRecherche
})
const textePartage = computed(() => `Aidez a retrouver ${nomComplet.value} sur UAfricas`)

// URLs de partage par reseau
const urlWhatsApp = computed(() =>
  `https://wa.me/?text=${encodeURIComponent(`${textePartage.value} ${urlPage.value}`)}`,
)
const urlFacebook = computed(() =>
  `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(urlPage.value)}`,
)
const urlTwitter = computed(() =>
  `https://twitter.com/intent/tweet?url=${encodeURIComponent(urlPage.value)}&text=${encodeURIComponent(textePartage.value)}`,
)
const urlLinkedIn = computed(() =>
  `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(urlPage.value)}`,
)

const partager = async (url: string) => {
  window.open(url, '_blank', 'noopener,noreferrer,width=600,height=400')
  const resultat = await incrementerPartage(props.slug)
  if (resultat) {
    compteur.value = resultat.compteur_partages
  }
}

const copierLien = async () => {
  try {
    await navigator.clipboard.writeText(urlPage.value)
    copieLienOk.value = true
    setTimeout(() => { copieLienOk.value = false }, 2000)
    const resultat = await incrementerPartage(props.slug)
    if (resultat) {
      compteur.value = resultat.compteur_partages
    }
  }
  catch {
    // Fallback si clipboard non disponible
  }
}

const reseaux = computed(() => [
  {
    nom: 'WhatsApp',
    url: urlWhatsApp.value,
    icon: ['fab', 'whatsapp'],
    couleur: 'bg-[#25D366] hover:bg-[#1da851]',
  },
  {
    nom: 'Facebook',
    url: urlFacebook.value,
    icon: ['fab', 'facebook'],
    couleur: 'bg-[#1877F2] hover:bg-[#0d65d9]',
  },
  {
    nom: 'X / Twitter',
    url: urlTwitter.value,
    icon: ['fab', 'twitter'],
    couleur: 'bg-black hover:bg-gray-800',
  },
  {
    nom: 'LinkedIn',
    url: urlLinkedIn.value,
    icon: ['fab', 'linkedin'],
    couleur: 'bg-[#0A66C2] hover:bg-[#084e96]',
  },
])
</script>

<template>
  <div class="mt-6">
    <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-sm text-gray-500 uppercase tracking-wide font-medium">
          Partager cet avis
        </h2>
        <span class="flex items-center gap-1.5 text-sm text-gray-500">
          <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-amber-700" />
          {{ compteur }} partage{{ compteur !== 1 ? 's' : '' }}
        </span>
      </div>

      <div class="flex flex-wrap gap-3">
        <!-- Boutons reseaux sociaux -->
        <button
          v-for="reseau in reseaux"
          :key="reseau.nom"
          type="button"
          class="flex items-center gap-2 px-4 py-2.5 text-white text-sm font-medium rounded-lg transition-colors"
          :class="reseau.couleur"
          :title="`Partager sur ${reseau.nom}`"
          @click="partager(reseau.url)"
        >
          <font-awesome-icon :icon="reseau.icon" />
          <span class="hidden sm:inline">{{ reseau.nom }}</span>
        </button>

        <!-- Bouton copier le lien -->
        <button
          type="button"
          class="flex items-center gap-2 px-4 py-2.5 text-sm font-medium rounded-lg transition-colors"
          :class="copieLienOk
            ? 'bg-green-100 text-green-700 border border-green-300'
            : 'bg-gray-100 text-gray-700 border border-gray-300 hover:bg-gray-200'"
          @click="copierLien"
        >
          <font-awesome-icon :icon="copieLienOk ? ['fas', 'check'] : ['fas', 'link']" />
          <span class="hidden sm:inline">{{ copieLienOk ? 'Copie !' : 'Copier le lien' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>
