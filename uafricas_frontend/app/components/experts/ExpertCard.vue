<script setup lang="ts">
import { type ExpertAPI, PROFILS_PROFESSIONNELS } from '~/composables/useExperts'

/**
 * Carte d'expert, portée sur les jetons de la refonte.
 *
 * Trois données INVENTÉES sont retirées au passage : elles s'affichaient pour
 * tout le monde, y compris pour un expert qui n'avait rien renseigné :
 *   - un badge « Disponible » codé en dur, sans rapport avec la situation
 *     professionnelle réellement déclarée ;
 *   - une note de repli `rating || 4.5` : un expert jamais noté était affiché
 *     à 4,5 étoiles. La note ne paraît plus que si `nombreNotes > 0` ;
 *   - une biographie de repli « Expert passionne avec une grande experience
 *     dans son domaine. » : du texte de remplissage, non accentué de surcroît.
 *
 * Le repli photo pointait vers `/images/default-avatar.jpg`, qui N'EXISTE PAS :
 * chaque expert sans photo déclenchait un 404. Il devient des initiales.
 *
 * Le survol ne recouvre plus la carte d'un calque qui répétait le nom, le
 * domaine et le même lien « Voir le profil » déjà présent en dessous.
 */
const props = defineProps<{ expert: ExpertAPI }>()

defineEmits<{ contact: [expert: ExpertAPI] }>()

const nomComplet = computed(() => `${props.expert.prenom} ${props.expert.nom}`)
const photo = computed(() => urlMedia(props.expert.photoURL))
const initiales = computed(() =>
  `${props.expert.prenom?.[0] ?? ''}${props.expert.nom?.[0] ?? ''}`.toUpperCase())

const domaine = computed(() => props.expert.expertiseInfo?.domaine || 'Expert')
const experience = computed(() => props.expert.expertiseInfo?.nbAnneesExperience ?? 0)

/** Note : affichée UNIQUEMENT si quelqu'un a réellement noté. */
const note = computed(() => {
  const info = props.expert.expertiseInfo
  if (!info || !info.nombreNotes) return null
  return { valeur: info.rating.toFixed(1), nombre: info.nombreNotes }
})

/** Situations déclarées, en libellés lisibles. */
const situations = computed(() =>
  (props.expert.situationProfessionnelle ?? [])
    .map(id => PROFILS_PROFESSIONNELS.find(p => p.id === id)?.label)
    .filter((l): l is string => Boolean(l) && l !== 'Tous les profils'))

const lieu = computed(() =>
  [props.expert.ville, props.expert.pays].filter(Boolean).join(', '))
</script>

<template>
  <article class="flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat">
    <NuxtLink :to="`/profil/${expert.id}`" class="group relative block aspect-[4/3] overflow-hidden bg-af-fond">
      <img
        v-if="photo"
        :src="photo"
        :alt="`Photo de ${nomComplet}`"
        class="size-full object-cover transition duration-300 group-hover:scale-105"
      />
      <span v-else class="grid size-full place-items-center text-4xl font-bold text-af-chocolat/40">
        {{ initiales }}
      </span>

      <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/75 to-transparent p-4">
        <h3 class="text-[17px]/[1.4] font-bold text-white">{{ nomComplet }}</h3>
        <p class="mt-0.5 flex flex-wrap items-center gap-x-4 gap-y-1 text-[12px]/[1.4] text-white/85">
          <span v-if="lieu" class="flex items-center gap-1.5">
            <font-awesome-icon icon="fa-solid fa-location-dot" />
            {{ lieu }}
          </span>
          <span class="flex items-center gap-1.5">
            <font-awesome-icon icon="fa-solid fa-briefcase" />
            {{ domaine }}
          </span>
        </p>
      </div>
    </NuxtLink>

    <div class="flex flex-1 flex-col gap-3 p-4">
      <div class="flex flex-wrap gap-2">
        <AfricansEtiquette v-if="experience > 0" ton="vert">
          {{ experience }} an{{ experience > 1 ? 's' : '' }} d'expérience
        </AfricansEtiquette>
        <AfricansEtiquette v-if="note">
          ★ {{ note.valeur }} ({{ note.nombre }})
        </AfricansEtiquette>
        <AfricansEtiquette v-for="s in situations" :key="s">{{ s }}</AfricansEtiquette>
      </div>

      <p v-if="expert.expertiseInfo?.biographie" class="line-clamp-3 text-[12px]/[1.4] text-af-corps">
        {{ expert.expertiseInfo.biographie }}
      </p>

      <div class="mt-auto flex gap-3 pt-1">
        <AfricansBouton class="flex-1" :vers="`/profil/${expert.id}`">Voir le profil</AfricansBouton>
        <button
          v-if="expert.email"
          type="button"
          class="grid size-11 shrink-0 place-items-center rounded-lg border border-af-bordure text-af-corps transition hover:border-af-chocolat hover:text-af-chocolat"
          title="Contacter"
          aria-label="Contacter"
          @click="$emit('contact', expert)"
        >
          <font-awesome-icon icon="fa-solid fa-envelope" />
        </button>
      </div>
    </div>
  </article>
</template>
