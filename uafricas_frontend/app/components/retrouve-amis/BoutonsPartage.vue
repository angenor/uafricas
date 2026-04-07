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
    <div class="rounded-2xl bg-white p-6 shadow-lg ring-1 ring-gray-200/60">
      <div class="flex items-center justify-between mb-5">
        <h2 class="flex items-center gap-2 text-sm font-bold uppercase tracking-widest text-gray-400">
          <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-amber-600" />
          Partager cet avis
        </h2>
        <span class="rounded-full bg-amber-50 px-3 py-1 text-xs font-semibold text-amber-700 ring-1 ring-amber-200/60">
          {{ compteur }} partage{{ compteur !== 1 ? 's' : '' }}
        </span>
      </div>

      <div class="flex flex-wrap gap-2.5">
        <!-- Boutons reseaux sociaux -->
        <button
          v-for="reseau in reseaux"
          :key="reseau.nom"
          type="button"
          class="flex items-center gap-2 rounded-xl px-4 py-2.5 text-sm font-medium text-white shadow-sm transition-all hover:shadow-md active:scale-95 cursor-pointer"
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
          class="flex items-center gap-2 rounded-xl px-4 py-2.5 text-sm font-medium shadow-sm transition-all active:scale-95 cursor-pointer"
          :class="copieLienOk
            ? 'bg-green-50 text-green-700 ring-1 ring-green-300'
            : 'bg-gray-50 text-gray-700 ring-1 ring-gray-200 hover:bg-gray-100'"
          @click="copierLien"
        >
          <font-awesome-icon :icon="copieLienOk ? ['fas', 'check'] : ['fas', 'link']" />
          <span class="hidden sm:inline">{{ copieLienOk ? 'Copie !' : 'Copier le lien' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>
