<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0"
      leave-active-class="transition duration-100 ease-in"
      leave-to-class="opacity-0"
    >
      <!-- Centrage par un conteneur INTERMÉDIAIRE en `min-h-full`, et non par
           `place-items-center` sur la boîte défilante. Avec le centrage direct,
           une modale plus HAUTE que la fenêtre voit son sommet rogné, et rien
           ne permet d'y remonter : le débordement d'un élément centré part des
           deux côtés, mais seul le bas est atteignable au défilement. Le titre
           et le bouton de fermeture de « Publier une annonce » étaient ainsi
           hors d'atteinte.
           Le remplissage vit sur le conteneur intermédiaire : c'est lui qui
           reçoit alors les clics hors de la boîte, y compris dans la marge. -->
      <div
        v-if="modelValue"
        class="fixed inset-0 overflow-y-auto bg-black/30 font-af"
        :class="couche === 'session' ? 'z-[10002]' : 'z-100'"
      >
        <div class="flex min-h-full items-center justify-center p-4" @click.self="fermer">
          <div
            ref="boite"
            role="dialog"
            aria-modal="true"
            :aria-labelledby="idTitre"
            class="relative w-full overflow-hidden rounded-[10px] bg-white shadow-xl"
            :class="taille === 'large' ? 'max-w-4xl' : 'max-w-[615px]'"
            tabindex="-1"
          >
            <!-- Bandeau de tête de 17 px. Vert à la création, chocolat pour
                 l'explication : la couleur DIT quelque chose, elle n'est pas
                 décorative. Un bandeau vert sur une modale d'onboarding
                 brouillerait le signal. -->
            <div class="h-[17px]" :class="ton === 'vert' ? 'bg-af-vert' : 'bg-af-chocolat'" />

            <button
              ref="boutonFermer"
              type="button"
              class="absolute top-8 right-6 grid size-6 place-items-center text-af-encre transition hover:opacity-60 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-af-chocolat"
              aria-label="Fermer"
              @click="fermer"
            >
              <font-awesome-icon icon="fa-solid fa-xmark" class="text-xl" />
            </button>

            <div class="p-8">
              <div class="flex items-center gap-3 pr-10">
                <font-awesome-icon v-if="icone" :icon="icone" class="size-6 text-af-chocolat" />
                <h2 :id="idTitre" class="text-[20px]/[1.4] font-bold">{{ titre }}</h2>
              </div>
              <p v-if="sousTitre" class="mt-1 text-[14px]/[1.4] text-af-atone italic">
                {{ sousTitre }}
              </p>

              <div class="mt-6"><slot /></div>

              <div v-if="$slots.actions" class="mt-8 flex items-center justify-end gap-6">
                <slot name="actions" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: boolean
  titre: string
  sousTitre?: string
  icone?: string
  /** `vert` = création de contenu, `chocolat` = explication / onboarding. */
  ton?: 'vert' | 'chocolat'
  /**
   * 615 px est la largeur de la maquette et le défaut. `large` est réservé
   * aux contenus qui ne s'y replient pas sans dommage : coques à onglets,
   * listes de fiches, formulaires à plusieurs dizaines de champs.
   */
  taille?: 'normale' | 'large'
  /**
   * `session` place la modale au-dessus de la salle Afrolang en plein écran,
   * qui occupe déjà z-10000. Le défaut, z-100, passe au-dessus de la barre
   * supérieure (z-50) et suffit partout ailleurs.
   */
  couche?: 'normale' | 'session'
}>(), { ton: 'vert', taille: 'normale', couche: 'normale' })

const emit = defineEmits<{ 'update:modelValue': [boolean] }>()

const idTitre = useId()
const boite = ref<HTMLElement | null>(null)
const boutonFermer = ref<HTMLElement | null>(null)

function fermer() {
  emit('update:modelValue', false)
}

function surTouche(e: KeyboardEvent) {
  if (e.key === 'Escape') fermer()
}

/**
 * Le focus est déplacé dans la modale à l'ouverture et rendu à l'élément qui
 * l'avait à la fermeture. Sans cela, la fermeture au clavier renvoie le focus
 * en tête de document et l'utilisateur perd sa place dans une liste longue.
 * Le défilement de l'arrière-plan est bloqué pour la même raison.
 */
const focusPrecedent = ref<HTMLElement | null>(null)

watch(() => props.modelValue, async (ouvert) => {
  if (import.meta.server) return

  if (ouvert) {
    focusPrecedent.value = document.activeElement as HTMLElement | null
    document.addEventListener('keydown', surTouche)
    document.body.style.overflow = 'hidden'
    await nextTick()
    boutonFermer.value?.focus()
  } else {
    document.removeEventListener('keydown', surTouche)
    document.body.style.overflow = ''
    focusPrecedent.value?.focus()
  }
})

onUnmounted(() => {
  if (import.meta.server) return
  document.removeEventListener('keydown', surTouche)
  document.body.style.overflow = ''
})
</script>
