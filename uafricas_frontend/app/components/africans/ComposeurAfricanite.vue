<template>
  <AfricansModale
    :model-value="modelValue"
    ton="vert"
    titre="Nouvelle africanité"
    :sous-titre="`Visible de vos ami(e)s pendant ${DUREE_HEURES} heures, puis elle disparaît.`"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <div class="flex flex-col gap-5">
      <AfricansBascule
        v-model="forme"
        libelle="Forme de l'africanité"
        :options="[
          { valeur: 'texte', libelle: 'Texte', icone: 'fa-solid fa-quote-left' },
          { valeur: 'image', libelle: 'Image', icone: 'fa-solid fa-image' },
          { valeur: 'video', libelle: 'Vidéo', icone: 'fa-solid fa-video' },
        ]"
      />

      <!-- Forme texte : aperçu en direct sur la couleur choisie. C'est ce que
           verront les lecteurs, il n'y a pas de raison de le leur cacher. -->
      <template v-if="forme === 'texte'">
        <div
          class="grid min-h-40 place-items-center rounded-[10px] p-6 text-center"
          :style="{ backgroundColor: couleur }"
        >
          <p class="text-[20px]/[1.4] font-bold whitespace-pre-line text-white">
            {{ texte || 'Votre texte apparaîtra ici' }}
          </p>
        </div>

        <AfricansChamp
          v-model="texte"
          libelle="Texte"
          type="textarea"
          placeholder="Quelques mots…"
          :aide="`${texte.length} / ${TEXTE_MAX_AFRICANITE} caractères`"
        />

        <div class="flex flex-col gap-2">
          <p class="text-[14px]/[1.4] text-af-atone italic">Couleur de fond</p>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="c in COULEURS_AFRICANITE"
              :key="c"
              type="button"
              class="size-9 rounded-full border-2 transition"
              :class="couleur === c ? 'border-af-encre scale-110' : 'border-transparent hover:scale-105'"
              :style="{ backgroundColor: c }"
              :aria-label="`Couleur ${c}`"
              :aria-pressed="couleur === c"
              @click="couleur = c"
            />
          </div>
        </div>
      </template>

      <!-- Formes image et vidéo -->
      <template v-else>
        <label
          class="flex cursor-pointer flex-col items-center gap-3 rounded-[10px] border-2 border-dashed border-af-bordure px-6 py-10 text-center transition hover:border-af-chocolat"
        >
          <font-awesome-icon
            :icon="forme === 'image' ? 'fa-solid fa-image' : 'fa-solid fa-video'"
            class="text-3xl text-af-chocolat"
          />
          <span class="text-[14px]/[1.4] font-bold">
            {{ fichier ? fichier.name : `Choisir ${forme === 'image' ? 'une image' : 'une vidéo courte'}` }}
          </span>
          <span v-if="fichier" class="text-[12px]/[1.4] text-af-atone">
            {{ (fichier.size / (1024 * 1024)).toFixed(1) }} Mo
          </span>
          <input
            type="file"
            class="sr-only"
            :accept="forme === 'image' ? 'image/*' : 'video/*'"
            @change="surFichier"
          />
        </label>

        <!-- La borne est dite AVANT l'envoi. Le serveur la fait respecter, mais
             découvrir un refus après avoir téléversé 40 Mo est une perte sèche. -->
        <p v-if="forme === 'video'" class="text-[12px]/[1.4] text-af-atone">
          Vidéo courte, 15 Mo au maximum. La plateforme ne recompresse rien.
        </p>
      </template>

      <AfricansChamp
        v-model="legende"
        libelle="Légende"
        placeholder="Facultative"
        icone="fa-solid fa-pen"
      />

      <p v-if="messageErreur" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5" />
        {{ messageErreur }}
      </p>
    </div>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="$emit('update:modelValue', false)"
      >
        Annuler
      </button>
      <AfricansBouton
        :desactive="enCours || !pretAPublier"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : undefined"
        @click="publier"
      >
        {{ enCours ? 'Publication…' : 'Publier' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import { COULEURS_AFRICANITE, TEXTE_MAX_AFRICANITE, type FormeAfricanite } from '~/composables/useAfricanite'

/**
 * Composeur d'africanité. Les trois formes de la décision Q3 de la spec, dans
 * une seule modale : basculer entre elles ne doit pas obliger à recommencer.
 */
const props = defineProps<{ modelValue: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [boolean], publiee: [] }>()

const DUREE_HEURES = 24

const { publierTexte, publierMedia, erreur } = useAfricanite()

const forme = ref<FormeAfricanite>('texte')
const texte = ref('')
const couleur = ref(COULEURS_AFRICANITE[0] ?? '#A74916')
const legende = ref('')
const fichier = ref<File | null>(null)
const enCours = ref(false)
const erreurLocale = ref<string | null>(null)

const messageErreur = computed(() => erreurLocale.value || erreur.value)

const pretAPublier = computed(() =>
  forme.value === 'texte' ? texte.value.trim().length > 0 : fichier.value !== null,
)

function surFichier(e: Event) {
  const cible = e.target as HTMLInputElement
  fichier.value = cible.files?.[0] ?? null
  erreurLocale.value = null
}

async function publier() {
  erreurLocale.value = null

  if (forme.value === 'texte' && texte.value.trim().length > TEXTE_MAX_AFRICANITE) {
    erreurLocale.value = `Texte trop long (${texte.value.trim().length} caractères, ${TEXTE_MAX_AFRICANITE} au maximum).`
    return
  }

  enCours.value = true
  const ok = forme.value === 'texte'
    ? await publierTexte(texte.value.trim(), couleur.value, legende.value.trim() || undefined)
    : await publierMedia(fichier.value!, forme.value, legende.value.trim() || undefined)
  enCours.value = false

  // La modale ne se referme QUE si la publication a abouti : la refermer sur
  // un échec jetterait la saisie avec elle.
  if (!ok) return
  emit('publiee')
  emit('update:modelValue', false)
}

// Remise à zéro à la FERMETURE, pas à l'ouverture : sur un échec, la modale
// reste ouverte et la saisie doit survivre.
watch(() => props.modelValue, (ouvert) => {
  if (ouvert) return
  forme.value = 'texte'
  texte.value = ''
  couleur.value = COULEURS_AFRICANITE[0] ?? '#A74916'
  legende.value = ''
  fichier.value = null
  erreurLocale.value = null
})
</script>
