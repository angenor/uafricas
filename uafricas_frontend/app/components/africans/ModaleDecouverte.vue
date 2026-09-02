<template>
  <AfricansModale
    :model-value="modelValue"
    ton="chocolat"
    :titre="titre"
    :sous-titre="sousTitre"
    :icone="icone"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <!-- La mascotte n'accompagne QUE le premier écran, et c'est ce que montre
         la maquette : elle accueille, elle n'illustre pas le reste. Elle est
         posée ici et non dans chaque modale : c'est la même pour tous les
         modules, malgré le nom de fichier hérité de Codimoi. -->
    <div class="flex flex-col gap-6" :class="etape === 0 && mascotte && 'sm:flex-row sm:items-center'">
      <div class="min-w-0 flex-1">
        <slot :etape="etape" />
      </div>
      <img
        v-if="etape === 0 && mascotte"
        src="/images/africans/illustrations/codimoi-personnage.svg"
        alt=""
        class="mx-auto w-36 shrink-0 sm:mx-0 sm:w-40"
      />
    </div>

    <template #actions>
      <!-- Les pastilles ne sont pas décoratives : elles disent combien
           d'écrans restent, et ce sont des boutons, revenir en arrière sans
           refermer la modale doit être possible. -->
      <div class="mr-auto flex items-center gap-2">
        <button
          v-for="i in nombreEtapes"
          :key="i"
          type="button"
          class="size-2 rounded-full transition"
          :class="i - 1 === etape ? 'bg-af-chocolat' : 'bg-af-bordure hover:bg-af-atone-2'"
          :aria-label="`Écran ${i} sur ${nombreEtapes}`"
          :aria-current="i - 1 === etape ? 'step' : undefined"
          @click="etape = i - 1"
        />
      </div>

      <AfricansBouton :icone="derniere ? undefined : 'fa-solid fa-arrow-right'" @click="suivant">
        {{ derniere ? 'J\'ai compris' : 'Suivant' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
/**
 * Modale de découverte « C'est quoi … ? » : un écran par pastille, bandeau
 * chocolat. La maquette la décline à l'identique sur Codimoi et Afrolang, d'où
 * le composant générique : seul le contenu des étapes change, et il est fourni
 * par le slot, qui reçoit l'index courant.
 */
const props = withDefaults(defineProps<{
  modelValue: boolean
  titre: string
  sousTitre?: string
  icone?: string
  nombreEtapes?: number
  /** Mascotte sur le premier écran. Désactivable pour un module qui aurait
   *  sa propre illustration. */
  mascotte?: boolean
}>(), { nombreEtapes: 3, mascotte: true })

const emit = defineEmits<{ 'update:modelValue': [boolean] }>()

const etape = ref(0)

const derniere = computed(() => etape.value >= props.nombreEtapes - 1)

function suivant() {
  if (derniere.value) emit('update:modelValue', false)
  else etape.value += 1
}

// Rouvrir la modale la reprend au premier écran : la reprendre là où elle
// avait été quittée donnerait un écran du milieu sans son contexte.
watch(() => props.modelValue, (ouvert) => {
  if (ouvert) etape.value = 0
})
</script>
