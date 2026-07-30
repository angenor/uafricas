<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { usePartageExterne, type OptionReseau } from '~/composables/usePartageExterne'

const props = withDefaults(defineProps<{
  /** Chemin relatif de la ressource (ex. `/evenements/abc-123`) */
  path: string
  /**
   * Identifiant de l'événement, nécessaire au traçage des partages externes.
   * Optionnel : sans lui le partage fonctionne, il n'est simplement pas compté.
   */
  evenementId?: string
  /** Titre de l'événement (utilisé pour le partage natif et le texte) */
  titre: string
  /** Texte d'accompagnement facultatif ; par défaut « Découvrez … sur UAfricas » */
  texte?: string
  /** `full` : barre de boutons ; `compact` : bouton unique + menu déroulant */
  variant?: 'full' | 'compact'
}>(), {
  variant: 'full',
})

// Origine résolue côté client ; repli sur le domaine de production pour le SSR
const origine = ref('https://www.africans-world.org')
const supporteWebShare = ref(false)
const copieLienOk = ref(false)
const menuOuvert = ref(false)

const urlPage = computed(() => `${origine.value}${props.path}`)
const textePartage = computed(() => props.texte || `Découvrez « ${props.titre} » sur UAfricas`)

// URLs de partage par réseau
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

// Telegram et e-mail complètent le catalogue : sans eux, la plateforme n'offrait
// que 4 réseaux et le seuil de 5 réseaux distincts du barème était structurellement
// inatteignable (R10).
const urlTelegram = computed(() =>
  `https://t.me/share/url?url=${encodeURIComponent(urlPage.value)}&text=${encodeURIComponent(textePartage.value)}`,
)
const urlEmail = computed(() =>
  `mailto:?subject=${encodeURIComponent(textePartage.value)}&body=${encodeURIComponent(`${textePartage.value} ${urlPage.value}`)}`,
)

const reseaux = computed<OptionReseau[]>(() => [
  { nom: 'WhatsApp', url: urlWhatsApp.value, icon: ['fab', 'whatsapp'], couleur: 'bg-[#25D366] hover:bg-[#1da851]', reseau: 'whatsapp' },
  { nom: 'Facebook', url: urlFacebook.value, icon: ['fab', 'facebook'], couleur: 'bg-[#1877F2] hover:bg-[#0d65d9]', reseau: 'facebook' },
  { nom: 'X / Twitter', url: urlTwitter.value, icon: ['fab', 'twitter'], couleur: 'bg-black hover:bg-gray-800', reseau: 'x' },
  { nom: 'LinkedIn', url: urlLinkedIn.value, icon: ['fab', 'linkedin'], couleur: 'bg-[#0A66C2] hover:bg-[#084e96]', reseau: 'linkedin' },
  { nom: 'Telegram', url: urlTelegram.value, icon: ['fab', 'telegram'], couleur: 'bg-[#229ED9] hover:bg-[#1b7fae]', reseau: 'telegram' },
  { nom: 'E-mail', url: urlEmail.value, icon: ['fas', 'envelope'], couleur: 'bg-gray-600 hover:bg-gray-700', reseau: 'email' },
])

const { tracerPartage } = usePartageExterne()

const partagerReseau = (r: OptionReseau) => {
  // `mailto:` ouvre le client de messagerie : `window.open` laisserait un onglet vide.
  if (r.url.startsWith('mailto:')) window.location.href = r.url
  else window.open(r.url, '_blank', 'noopener,noreferrer,width=600,height=500')
  menuOuvert.value = false

  // Traçage APRÈS l'ouverture, best-effort.
  if (r.reseau && props.evenementId) tracerPartage('evenement', props.evenementId, r.reseau)
}

const partageNatif = async () => {
  if (typeof navigator !== 'undefined' && navigator.share) {
    try {
      await navigator.share({ title: props.titre, text: textePartage.value, url: urlPage.value })
    }
    catch {
      // Partage annulé par l'utilisateur — sans effet
    }
  }
  menuOuvert.value = false
}

const copierLien = async () => {
  try {
    await navigator.clipboard.writeText(urlPage.value)
    copieLienOk.value = true
    setTimeout(() => { copieLienOk.value = false }, 2000)
  }
  catch {
    // Presse-papiers indisponible — sans effet
  }
}

// Fermeture du menu compact au clic extérieur
const racine = ref<HTMLElement | null>(null)
const onClicExterieur = (e: MouseEvent) => {
  if (menuOuvert.value && racine.value && !racine.value.contains(e.target as Node)) {
    menuOuvert.value = false
  }
}

onMounted(() => {
  if (typeof window !== 'undefined') origine.value = window.location.origin
  supporteWebShare.value = typeof navigator !== 'undefined' && !!navigator.share
  document.addEventListener('click', onClicExterieur)
})
onBeforeUnmount(() => document.removeEventListener('click', onClicExterieur))
</script>

<template>
  <!-- Variante compacte : bouton icône + menu déroulant -->
  <div v-if="variant === 'compact'" ref="racine" class="relative inline-block">
    <button
      type="button"
      class="flex items-center justify-center w-9 h-9 rounded-full bg-white/90 text-gray-600 shadow-sm ring-1 ring-gray-200 hover:text-custom-green hover:ring-custom-green/40 transition-all active:scale-95 cursor-pointer"
      title="Partager cet événement"
      aria-label="Partager cet événement"
      @click.stop.prevent="menuOuvert = !menuOuvert"
    >
      <font-awesome-icon :icon="['fas', 'share-nodes']" />
    </button>

    <Transition name="menu-fade">
      <div
        v-if="menuOuvert"
        class="absolute left-0 z-30 mt-2 w-48 rounded-xl bg-white p-2 shadow-xl ring-1 ring-gray-200"
        @click.stop
      >
        <button
          v-for="reseau in reseaux"
          :key="reseau.nom"
          type="button"
          class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm text-gray-700 hover:bg-gray-50 cursor-pointer"
          @click="partagerReseau(reseau)"
        >
          <font-awesome-icon :icon="reseau.icon" class="w-4 text-center" />
          {{ reseau.nom }}
        </button>

        <button
          v-if="supporteWebShare"
          type="button"
          class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm text-gray-700 hover:bg-gray-50 cursor-pointer"
          @click="partageNatif"
        >
          <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 text-center" />
          Plus…
        </button>

        <button
          type="button"
          class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm cursor-pointer"
          :class="copieLienOk ? 'text-green-700' : 'text-gray-700 hover:bg-gray-50'"
          @click="copierLien"
        >
          <font-awesome-icon :icon="copieLienOk ? ['fas', 'check'] : ['fas', 'link']" class="w-4 text-center" />
          {{ copieLienOk ? 'Lien copié !' : 'Copier le lien' }}
        </button>
      </div>
    </Transition>
  </div>

  <!-- Variante complète : barre de boutons -->
  <div v-else ref="racine">
    <div class="rounded-2xl bg-white p-5 shadow-sm ring-1 ring-gray-200/60">
      <h2 class="mb-4 flex items-center gap-2 text-sm font-bold uppercase tracking-widest text-gray-400">
        <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-custom-green" />
        Partager cet événement
      </h2>

      <div class="flex flex-wrap gap-2.5">
        <button
          v-for="reseau in reseaux"
          :key="reseau.nom"
          type="button"
          class="flex items-center gap-2 rounded-xl px-4 py-2.5 text-sm font-medium text-white shadow-sm transition-all hover:shadow-md active:scale-95 cursor-pointer"
          :class="reseau.couleur"
          :title="`Partager sur ${reseau.nom}`"
          @click="partagerReseau(reseau)"
        >
          <font-awesome-icon :icon="reseau.icon" />
          <span class="hidden sm:inline">{{ reseau.nom }}</span>
        </button>

        <button
          v-if="supporteWebShare"
          type="button"
          class="flex items-center gap-2 rounded-xl px-4 py-2.5 text-sm font-medium text-white bg-custom-chocolat hover:bg-custom-chocolat/90 shadow-sm transition-all active:scale-95 cursor-pointer"
          title="Plus d'options de partage"
          @click="partageNatif"
        >
          <font-awesome-icon :icon="['fas', 'share-nodes']" />
          <span class="hidden sm:inline">Plus…</span>
        </button>

        <button
          type="button"
          class="flex items-center gap-2 rounded-xl px-4 py-2.5 text-sm font-medium shadow-sm transition-all active:scale-95 cursor-pointer"
          :class="copieLienOk
            ? 'bg-green-50 text-green-700 ring-1 ring-green-300'
            : 'bg-gray-50 text-gray-700 ring-1 ring-gray-200 hover:bg-gray-100'"
          @click="copierLien"
        >
          <font-awesome-icon :icon="copieLienOk ? ['fas', 'check'] : ['fas', 'link']" />
          <span class="hidden sm:inline">{{ copieLienOk ? 'Copié !' : 'Copier le lien' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.menu-fade-enter-active,
.menu-fade-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}
.menu-fade-enter-from,
.menu-fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
