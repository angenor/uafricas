<template>
  <AfricansModale
    :model-value="modelValue"
    ton="chocolat"
    :titre="salle?.titre ?? 'Salle'"
    icone="fa-solid fa-language"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template v-if="salle">
      <!-- Vignette en tête, comme sur la maquette : elle rattache la modale à
           la carte d'où elle a été ouverte. -->
      <div class="-mt-2 mb-6 aspect-[16/6] w-full overflow-hidden rounded-[10px] bg-af-bordure">
        <img v-if="salle.image_couverture_url" :src="salle.image_couverture_url" alt="" class="size-full object-cover" />
      </div>

      <div class="flex flex-wrap items-center gap-x-6 gap-y-2 text-[14px]/[1.4]">
        <span class="flex items-center gap-2">
          <span class="size-2 rounded-full" :class="enDirect ? 'bg-af-live' : 'bg-af-atone-2'" />
          {{ enDirect ? 'Live en cours' : 'Non démarrée' }}
        </span>
        <span v-if="salle.nombre_moderateurs_attitres" class="flex items-center gap-2 text-af-corps">
          <font-awesome-icon icon="fa-solid fa-user-shield" class="text-af-chocolat" />
          {{ salle.nombre_moderateurs_attitres }} modérateur{{ salle.nombre_moderateurs_attitres > 1 ? 's' : '' }} attitré{{ salle.nombre_moderateurs_attitres > 1 ? 's' : '' }}
        </span>
        <span class="flex items-center gap-2 text-af-corps">
          <font-awesome-icon icon="fa-solid fa-door-open" class="text-af-chocolat" />
          {{ salle.nombre_salles_privees }} cours privé{{ salle.nombre_salles_privees > 1 ? 's' : '' }}
        </span>
      </div>

      <!-- Fermeture administrative : première information de la modale quand
           elle s'applique, parce qu'elle invalide toutes les suivantes. -->
      <p
        v-if="salle.desactivee_admin"
        class="mt-6 flex items-start gap-3 rounded-[10px] border border-af-live/30 bg-af-live/[0.05] px-4 py-3 text-[14px]/[1.4] text-af-corps"
      >
        <font-awesome-icon icon="fa-solid fa-ban" class="mt-1 text-af-live" />
        <span>
          <strong class="font-bold">Salle fermée par l'administration.</strong>
          <template v-if="salle.desactivee_admin.motif"> {{ salle.desactivee_admin.motif }}</template>
        </span>
      </p>

      <template v-if="salle.description">
        <p class="mt-6 text-[14px]/[1.4] text-af-atone italic">Description</p>
        <p class="mt-1 text-[14px]/[1.4] whitespace-pre-line text-af-corps">{{ salle.description }}</p>
      </template>

      <dl class="mt-6 grid gap-x-8 gap-y-3 sm:grid-cols-2">
        <div v-if="territoires" class="flex flex-col gap-1">
          <dt class="text-[14px]/[1.4] text-af-atone italic">Territoires d'origine</dt>
          <dd class="flex items-center gap-2 text-[14px]/[1.4]">
            <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-chocolat" />
            {{ territoires }}
          </dd>
        </div>

        <div v-if="groupeEthnique" class="flex flex-col gap-1">
          <dt class="text-[14px]/[1.4] text-af-atone italic">Groupe ethnique</dt>
          <dd class="text-[14px]/[1.4]">{{ groupeEthnique }}</dd>
        </div>

        <div v-if="salle.langue_cible" class="flex flex-col gap-1">
          <dt class="text-[14px]/[1.4] text-af-atone italic">Langue cible</dt>
          <dd class="text-[14px]/[1.4]">{{ salle.langue_cible }}</dd>
        </div>

        <div v-if="salle.langue_code" class="flex flex-col gap-1">
          <dt class="text-[14px]/[1.4] text-af-atone italic">Code de la langue</dt>
          <dd class="text-[14px]/[1.4]">{{ salle.langue_code }}</dd>
        </div>

        <div v-if="salle.alphabet" class="flex flex-col gap-1">
          <dt class="text-[14px]/[1.4] text-af-atone italic">Alphabet</dt>
          <dd class="text-[14px]/[1.4]">{{ salle.alphabet }}</dd>
        </div>

        <div v-if="salle.dictionnaire_url" class="flex flex-col gap-1">
          <dt class="text-[14px]/[1.4] text-af-atone italic">Dictionnaire</dt>
          <dd>
            <a
              :href="salle.dictionnaire_url"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center gap-2 text-[14px]/[1.4] text-af-chocolat underline underline-offset-4 hover:opacity-70"
            >
              <font-awesome-icon icon="fa-solid fa-book-open" />
              Consulter
            </a>
          </dd>
        </div>
      </dl>

      <div v-if="salle.administrateurs?.length" class="mt-6 flex flex-col gap-2">
        <p class="text-[14px]/[1.4] text-af-atone italic">Administrateurs de la salle</p>
        <ul class="flex flex-wrap gap-x-6 gap-y-2">
          <li
            v-for="admin in salle.administrateurs"
            :key="admin.utilisateur_id"
            class="flex items-center gap-2 text-[14px]/[1.4]"
          >
            <AfricansAvatar :nom="`${admin.prenom} ${admin.nom}`" :src="urlMedia(admin.photo_url)" :taille="24" />
            {{ admin.prenom }} {{ admin.nom }}
          </li>
        </ul>
      </div>
    </template>

    <template #actions>
      <AfricansBouton
        v-if="salle"
        :desactive="!!salle.desactivee_admin"
        :icone="salle.desactivee_admin ? 'fa-solid fa-ban' : (enDirect ? 'fa-solid fa-video' : 'fa-solid fa-plus')"
        @click="$emit('agir', salle.id)"
      >
        {{ salle.desactivee_admin ? 'Salle fermée' : (enDirect ? 'Suivre le live' : 'Démarrer') }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import type { SalleAPI } from '~/composables/useAfrolang'

/**
 * Fiche d'une salle publique (« Infos salle Afrolang » de la maquette).
 *
 * Elle ne déclenche AUCUNE requête : tout ce qu'elle montre est déjà porté par
 * `SalleAPI`, donc déjà chargé par la liste. C'est aussi la seule surface de
 * l'application où l'alphabet, le dictionnaire et le groupe ethnique d'une
 * salle sont encore visibles : l'ancienne page `/afrolang/[id]` qui les portait
 * a été réduite à une redirection vers la session.
 */
const props = defineProps<{
  modelValue: boolean
  salle: SalleAPI | null
}>()

defineEmits<{ 'update:modelValue': [boolean], agir: [salleId: string] }>()

const enDirect = computed(() => (props.salle?.sessions_en_cours ?? 0) > 0)

const territoires = computed(() =>
  props.salle?.pays_origine?.map(p => p.nom).join(', ') || null)

/** Le groupe référencé prime ; `groupe_ethnique_libre` est le repli des salles
 *  nées d'une proposition « Autre », qui n'en visent aucun du référentiel. */
const groupeEthnique = computed(() =>
  props.salle?.groupe_ethnique?.nom || props.salle?.groupe_ethnique_libre || null)
</script>
