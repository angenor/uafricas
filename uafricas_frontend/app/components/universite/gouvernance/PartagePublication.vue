<script setup lang="ts">
const props = defineProps<{
  /** Chemin de la page section (ex: /universite/gouvernance/ideaforces) */
  path: string
  /** Identifiant de la publication, ajouté en ?pub= */
  id: string
  /** Titre de la publication, utilisé comme texte de partage */
  titre: string
}>()

// Domaine de production : on ne partage jamais une URL localhost.
const BASE_URL = 'https://www.africans-world.org'

const ouvert = ref(false)
const copieOk = ref(false)

const urlPartage = computed(() => `${BASE_URL}${props.path}?pub=${props.id}`)
const textePartage = computed(() => props.titre)

const reseaux = computed(() => [
  {
    nom: 'WhatsApp',
    url: `https://wa.me/?text=${encodeURIComponent(`${textePartage.value} ${urlPartage.value}`)}`,
    icon: ['fab', 'whatsapp'],
    couleur: 'text-[#25D366]',
  },
  {
    nom: 'Facebook',
    url: `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(urlPartage.value)}`,
    icon: ['fab', 'facebook'],
    couleur: 'text-[#1877F2]',
  },
  {
    nom: 'X / Twitter',
    url: `https://twitter.com/intent/tweet?url=${encodeURIComponent(urlPartage.value)}&text=${encodeURIComponent(textePartage.value)}`,
    icon: ['fab', 'twitter'],
    couleur: 'text-black',
  },
  {
    nom: 'LinkedIn',
    url: `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(urlPartage.value)}`,
    icon: ['fab', 'linkedin'],
    couleur: 'text-[#0A66C2]',
  },
])

function partager(url: string) {
  window.open(url, '_blank', 'noopener,noreferrer,width=600,height=500')
  ouvert.value = false
}

async function copierLien() {
  try {
    await navigator.clipboard.writeText(urlPartage.value)
    copieOk.value = true
    setTimeout(() => { copieOk.value = false }, 2000)
  }
  catch {
    // Clipboard indisponible : on laisse le menu ouvert
  }
}
</script>

<template>
  <div class="relative">
    <button
      type="button"
      class="flex items-center gap-1.5 hover:text-gray-700 transition"
      :title="'Partager'"
      @click.stop="ouvert = !ouvert"
    >
      <font-awesome-icon :icon="['fas', 'share-nodes']" />
      <span class="hidden sm:inline">Partager</span>
    </button>

    <!-- Backdrop pour fermer au clic extérieur -->
    <div v-if="ouvert" class="fixed inset-0 z-30" @click.stop="ouvert = false"></div>

    <!-- Menu -->
    <div
      v-if="ouvert"
      class="absolute right-0 bottom-full mb-2 z-40 w-52 rounded-xl bg-white shadow-xl ring-1 ring-gray-200 p-2"
      @click.stop
    >
      <button
        v-for="reseau in reseaux"
        :key="reseau.nom"
        type="button"
        class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 transition"
        @click="partager(reseau.url)"
      >
        <font-awesome-icon :icon="reseau.icon" :class="reseau.couleur" class="w-4" />
        {{ reseau.nom }}
      </button>

      <div class="my-1 border-t border-gray-100"></div>

      <button
        type="button"
        class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition"
        :class="copieOk ? 'text-green-700 bg-green-50' : 'text-gray-700 hover:bg-gray-50'"
        @click="copierLien"
      >
        <font-awesome-icon :icon="copieOk ? ['fas', 'check'] : ['fas', 'link']" class="w-4" />
        {{ copieOk ? 'Lien copié !' : 'Copier le lien' }}
      </button>
    </div>
  </div>
</template>
