<template>
  <form class="flex flex-col gap-4 rounded-[10px] border border-af-bordure bg-af-surface p-5" @submit.prevent="envoyer">
    <h3 class="text-[17px]/[1.4] font-bold text-af-encre">{{ remplace ? 'Remplacer ma photo' : 'Déposer ma photo' }}</h3>

    <label class="flex flex-col gap-2">
      <span class="text-[14px]/[1.4] font-bold text-af-encre">Photo (JPEG, PNG ou WebP)</span>
      <div
        class="grid min-h-44 place-items-center overflow-hidden rounded-lg border-2 border-dashed border-af-bordure bg-af-fond/40"
        :class="apercu ? '' : 'p-6'"
      >
        <img v-if="apercu" :src="apercu" alt="Aperçu de la photo choisie" class="max-h-72 w-full object-contain">
        <span v-else class="text-center text-[14px]/[1.5] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-image" class="mb-2 block text-2xl text-af-atone" />
          Choisissez une photo prise par vous
        </span>
      </div>
      <input type="file" accept="image/jpeg,image/png,image/webp" class="text-[14px] text-af-corps file:mr-3 file:rounded-md file:border-0 file:bg-af-chocolat/10 file:px-3 file:py-2 file:font-bold file:text-af-chocolat" @change="choisir">
    </label>

    <label class="flex flex-col gap-1.5">
      <span class="flex justify-between text-[14px]/[1.4] font-bold text-af-encre">
        Légende <span class="font-normal text-af-atone">{{ legende.length }} / 200</span>
      </span>
      <input v-model="legende" type="text" maxlength="200" class="rounded-lg border border-af-bordure bg-af-surface px-3 py-2.5 text-[15px] text-af-encre focus-visible:outline-2 focus-visible:outline-af-chocolat" placeholder="Le plat de ma grand-mère, à Kumasi">
    </label>

    <p class="flex items-start gap-2 text-[13px]/[1.5] text-af-corps">
      <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-0.5 text-af-atone" />
      Votre photo sera relue par l'équipe avant d'être publiée. Vous serez prévenu de la décision.
    </p>

    <p v-if="erreur" class="text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>

    <div class="flex justify-end gap-3">
      <AfricansBouton v-if="remplace" variante="secondaire" @click="$emit('annuler')">Annuler</AfricansBouton>
      <AfricansBouton type="submit" icone="fa-solid fa-paper-plane" :desactive="envoi || (!fichier && !remplace)" :tourne="envoi">
        {{ remplace ? 'Envoyer à la relecture' : 'Déposer' }}
      </AfricansBouton>
    </div>
  </form>
</template>

<script setup lang="ts">
import { messageErreurJeu } from '~/composables/useJeu'
import type { MaParticipationAPI } from '~/composables/useConcours'

/**
 * Dépôt d'une photo de concours (feature 014). Le serveur contrôle et
 * normalise la photo (métadonnées retirées) ; ses messages d'erreur sont
 * affichés tels quels, ils disent quoi corriger.
 */
const props = defineProps<{
  concoursId: string
  /** Participation remplacée (en attente ou refusée), le cas échéant. */
  remplace?: MaParticipationAPI | null
}>()

const emit = defineEmits<{ depose: [participation: MaParticipationAPI], annuler: [] }>()

const { deposer, remplacer } = useConcours()

const fichier = ref<File | null>(null)
const apercu = ref<string | null>(props.remplace ? urlMedia(props.remplace.media_url) : null)
const legende = ref(props.remplace?.legende ?? '')
const envoi = ref(false)
const erreur = ref('')

const choisir = (e: Event) => {
  const f = (e.target as HTMLInputElement).files?.[0] ?? null
  fichier.value = f
  if (apercu.value?.startsWith('blob:')) URL.revokeObjectURL(apercu.value)
  apercu.value = f ? URL.createObjectURL(f) : (props.remplace ? urlMedia(props.remplace.media_url) : null)
}

const envoyer = async () => {
  erreur.value = ''
  envoi.value = true
  try {
    const p = props.remplace
      ? await remplacer(props.concoursId, props.remplace.id, fichier.value, legende.value.trim())
      : await deposer(props.concoursId, fichier.value!, legende.value.trim())
    if (p) emit('depose', p)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Votre photo n\'a pas pu être envoyée.')
  }
  finally {
    envoi.value = false
  }
}

onBeforeUnmount(() => { if (apercu.value?.startsWith('blob:')) URL.revokeObjectURL(apercu.value) })
</script>
