<script setup lang="ts">
// Badge de statut d'engagement — Tailwind pur (réutilisable : profil + sous contenus)
import { computed, ref, watch, onMounted } from 'vue'
import { useEngagement, type NiveauInfo } from '~/composables/useEngagement'

const props = withDefaults(defineProps<{
  /** Niveau déjà connu (évite un appel réseau) */
  niveau?: NiveauInfo | null
  /** Sinon, l'id du membre dont on récupère le niveau */
  utilisateurId?: string
  /** Taille du badge */
  taille?: 'sm' | 'md'
}>(), {
  niveau: null,
  utilisateurId: '',
  taille: 'md',
})

const { obtenirNiveau } = useEngagement()
const niveauLocal = ref<NiveauInfo | null>(props.niveau)

const chargerSiBesoin = async () => {
  if (!niveauLocal.value && props.utilisateurId) {
    try {
      niveauLocal.value = await obtenirNiveau(props.utilisateurId)
    } catch {
      niveauLocal.value = null
    }
  }
}

watch(() => props.niveau, (v) => { if (v) niveauLocal.value = v })
watch(() => props.utilisateurId, chargerSiBesoin)
onMounted(chargerSiBesoin)

// Mapping couleur (jeton libre en base : gray / amber / yellow / slate…) → classes
// Tailwind. Le libellé, la couleur et l'icône viennent TOUS de l'API : aucun nom
// de statut n'est écrit en dur ici, sans quoi renommer « Premium » en
// back-office ne changerait rien à l'écran (FR-032).
//
// Les quatre statuts livrés — Membre Africans (gray), Premium (amber),
// Gold (yellow), Platinum (slate) — ne sont qu'un paramétrage parmi d'autres :
// en ajouter un cinquième ne demande aucune livraison, seulement un jeton connu
// de cette table.
const CLASSES: Record<string, string> = {
  gray: 'bg-gray-100 text-gray-700 ring-gray-300',
  amber: 'bg-amber-100 text-amber-800 ring-amber-300',
  yellow: 'bg-yellow-100 text-yellow-800 ring-yellow-400',
  slate: 'bg-slate-800 text-white ring-slate-500',
  green: 'bg-custom-green/10 text-custom-green ring-custom-green/40',
  rose: 'bg-rose-100 text-rose-700 ring-rose-300',
  sky: 'bg-sky-100 text-sky-800 ring-sky-300',
  violet: 'bg-violet-100 text-violet-800 ring-violet-300',
}

const classesCouleur = computed(() =>
  CLASSES[niveauLocal.value?.badge_couleur || 'gray'] || CLASSES.gray,
)
const classesTaille = computed(() =>
  props.taille === 'sm' ? 'text-[11px] px-2 py-0.5 gap-1' : 'text-xs px-2.5 py-1 gap-1.5',
)
const icone = computed(() => `fa-solid fa-${niveauLocal.value?.badge_icone || 'user'}`)
</script>

<template>
  <span
    v-if="niveauLocal"
    class="inline-flex items-center font-semibold rounded-full ring-1"
    :class="[classesCouleur, classesTaille]"
    :title="`Statut : ${niveauLocal.libelle}`"
  >
    <font-awesome-icon :icon="icone" />
    {{ niveauLocal.libelle }}
  </span>
</template>
