<script setup lang="ts">
// Bouton admin « Mettre en avant » (règle d'engagement +5). Autonome : vérifie
// l'état au montage puis bascule via le composable useAdminEngagement.
// À déposer dans les pages d'édition admin des contributions ayant un auteur
// unique (codimoi, factcheck, bad_habit, idea_force, video/piste).
const props = defineProps<{
  typeObjet: string
  objetId: string
}>()

const { statutMiseEnAvant, mettreEnAvant, retirerMiseEnAvant } = useAdminEngagement()

const misEnAvant = ref(false)
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref<string | null>(null)

const rafraichir = async () => {
  chargement.value = true
  erreur.value = null
  try {
    misEnAvant.value = await statutMiseEnAvant(props.typeObjet, props.objetId)
  } catch {
    erreur.value = 'Impossible de charger l’état de mise en avant'
  } finally {
    chargement.value = false
  }
}

const basculer = async () => {
  if (enCours.value) return
  enCours.value = true
  erreur.value = null
  try {
    if (misEnAvant.value) {
      await retirerMiseEnAvant(props.typeObjet, props.objetId)
      misEnAvant.value = false
    } else {
      await mettreEnAvant(props.typeObjet, props.objetId)
      misEnAvant.value = true
    }
  } catch {
    erreur.value = 'Action impossible'
  } finally {
    enCours.value = false
  }
}

onMounted(rafraichir)
watch(() => props.objetId, rafraichir)
</script>

<template>
  <div class="flex flex-col gap-1">
    <button
      type="button"
      class="btn btn-sm gap-2"
      :class="misEnAvant ? 'btn-warning' : 'btn-outline'"
      :disabled="chargement || enCours"
      @click="basculer"
    >
      <span v-if="enCours || chargement" class="loading loading-spinner loading-xs" />
      <font-awesome-icon v-else :icon="['fas', 'star']" />
      {{ misEnAvant ? 'Mise en avant (retirer)' : 'Mettre en avant' }}
    </button>
    <p v-if="misEnAvant" class="text-xs opacity-60">
      +5 points attribués à l'auteur (une seule fois).
    </p>
    <p v-if="erreur" class="text-xs text-error">{{ erreur }}</p>
  </div>
</template>
