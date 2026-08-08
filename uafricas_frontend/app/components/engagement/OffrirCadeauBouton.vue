<script setup lang="ts">
/**
 * Point d'entrée réutilisable vers l'offre de cadeau — Tailwind v4 pur.
 *
 * Monté sur les sept familles de contenus et sur les profils. Il se masque
 * lui-même dans les trois cas où offrir n'aurait pas de sens :
 * — visiteur non connecté (le serveur refuserait, autant ne rien promettre) ;
 * — le membre est l'auteur du contenu (l'auto-cadeau est refusé, et une
 *   contrainte `CHECK` le rend impossible en base) ;
 * — la famille n'a pas d'auteur enregistré (`site_touristique`,
 *   `secteur_developpement`) : proposer le bouton ne mènerait qu'à un 409.
 */
import { computed, ref } from 'vue'
import { FAMILLES_CADEAU } from '~/composables/useCadeaux'
import { useUserStore } from '~/stores/user'

const props = withDefaults(defineProps<{
  /** Famille du contenu, ou `'profil'`. */
  typeObjet: string
  /** Identifiant du contenu, ou du membre visé si `typeObjet === 'profil'`. */
  objetId: string
  /** Auteur du contenu, quand la page le connaît : sert à masquer l'auto-cadeau. */
  auteurId?: string
  /** Nom du destinataire, affiché dans la modale. */
  destinataire?: string
  taille?: 'sm' | 'md'
  /** Bouton plein plutôt que contour. */
  plein?: boolean
}>(), {
  auteurId: '',
  destinataire: '',
  taille: 'md',
  plein: false,
})

const emit = defineEmits<{ (e: 'offert', points: number): void }>()

const userStore = useUserStore()
const ouvert = ref(false)

const familleEligible = computed(() =>
  (FAMILLES_CADEAU as readonly string[]).includes(props.typeObjet),
)

/**
 * Pour un profil, la cible EST le bénéficiaire : `objetId` suffit à détecter
 * l'auto-cadeau, sans que la page ait à transmettre un `auteurId`.
 */
const estSonPropreContenu = computed(() => {
  const moi = userStore.user?.id
  if (!moi) return false
  if (props.typeObjet === 'profil' || props.typeObjet === 'biblio_humaine') {
    return props.objetId === moi
  }
  return !!props.auteurId && props.auteurId === moi
})

const visible = computed(() =>
  !!userStore.accessToken
  && !!props.objetId
  && familleEligible.value
  && !estSonPropreContenu.value,
)

const classesTaille = computed(() =>
  props.taille === 'sm' ? 'px-2.5 py-1 text-xs gap-1.5' : 'px-4 py-2 text-sm gap-2',
)

const classesStyle = computed(() =>
  props.plein
    ? 'bg-custom-chocolat text-white hover:bg-custom-chocolat/90'
    : 'border border-custom-chocolat/30 text-custom-chocolat hover:bg-custom-chocolat/5',
)

const onOffert = (points: number) => {
  ouvert.value = false
  emit('offert', points)
}
</script>

<template>
  <template v-if="visible">
    <button
      type="button"
      class="inline-flex cursor-pointer items-center rounded-lg font-medium transition-colors"
      :class="[classesTaille, classesStyle]"
      @click="ouvert = true"
    >
      <font-awesome-icon icon="fa-solid fa-gift" />
      Offrir un cadeau
    </button>

    <EngagementOffrirCadeauModal
      :is-open="ouvert"
      :type-objet="typeObjet"
      :objet-id="objetId"
      :destinataire="destinataire"
      @close="ouvert = false"
      @offert="onOffert"
    />
  </template>
</template>
