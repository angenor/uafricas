<script setup lang="ts">
/**
 * Les idées de contenu et demandes d'animation reçues par un support, arbitrées
 * par ses co-détenteurs (US6 : FR-045, FR-047).
 *
 * L'API et le composable existaient déjà ; il manquait ce point de montage, si
 * bien qu'une demande d'animation ne pouvait être acceptée depuis nulle part et
 * que FR-045 restait inatteignable.
 *
 * Deux natures de demande, deux conséquences :
 *   • une IDÉE acceptée ne crée aucun objet : elle vaut accusé de réception ;
 *   • une DEMANDE D'ANIMATION acceptée ajoute son auteur aux co-détenteurs du
 *     support, et lui ouvre donc la grille. C'est une décision d'équipe, pas un
 *     simple accusé de réception : le libellé du bouton le dit.
 */
import { useMediaDetention, type TypeSupportMedia } from '~/composables/useMediaDetention'
import type { PropositionMediaAPI } from '~/composables/useMediaProposition'

const props = defineProps<{
  typeSupport: TypeSupportMedia
  supportId: string
  /** Seul le propriétaire arbitre : un co-détenteur consulte sans décider. */
  monRole: string
}>()

const { listerPropositionsSupport, deciderProposition, erreur } = useMediaDetention()

const demandes = ref<PropositionMediaAPI[]>([])
const chargement = ref(false)
const traitement = ref<string | null>(null)

/** Motif de refus, saisi par demande. Un refus sans motif n'apprend rien. */
const motifs = ref<Record<string, string>>({})
const refusOuvert = ref<string | null>(null)

const peutDecider = computed(() => props.monRole === 'proprietaire')

const charger = async () => {
  chargement.value = true
  demandes.value = await listerPropositionsSupport(
    props.typeSupport,
    props.supportId,
    'en_attente',
  )
  chargement.value = false
}

onMounted(charger)

const auteur = (d: PropositionMediaAPI) =>
  `${d.auteur_prenom ?? ''} ${d.auteur_nom ?? ''}`.trim() || 'Membre'

const titre = (d: PropositionMediaAPI) => {
  const donnees = d.donnees as Record<string, unknown>
  return (donnees.titre as string)
    || (donnees.motivation as string)
    || 'Demande sans titre'
}

const dateFormatee = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })

const accepter = async (d: PropositionMediaAPI) => {
  if (traitement.value) return
  traitement.value = d.id
  const ok = await deciderProposition(d.id, 'accepter')
  traitement.value = null
  if (ok) await charger()
}

const refuser = async (d: PropositionMediaAPI) => {
  const motif = (motifs.value[d.id] ?? '').trim()
  if (motif.length < 10) return
  traitement.value = d.id
  const ok = await deciderProposition(d.id, 'refuser', motif)
  traitement.value = null
  if (ok) {
    refusOuvert.value = null
    motifs.value[d.id] = ''
    await charger()
  }
}
</script>

<template>
  <div>
    <p v-if="erreur" class="mb-3 rounded-lg bg-red-50 px-4 py-2 text-sm text-red-700">
      {{ erreur }}
    </p>

    <p v-if="chargement" class="py-6 text-center text-sm text-gray-500">
      Chargement des demandes…
    </p>

    <p v-else-if="demandes.length === 0" class="py-6 text-center text-sm text-gray-500">
      Aucune demande en attente. Les idées de contenu et les demandes d'animation
      déposées par les visiteurs apparaîtront ici.
    </p>

    <ul v-else class="space-y-4">
      <li
        v-for="demande in demandes"
        :key="demande.id"
        class="rounded-2xl border border-gray-200 bg-white p-4"
      >
        <div class="flex flex-wrap items-start justify-between gap-2">
          <span
            class="inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-semibold"
            :class="demande.type_objet === 'animation_programme'
              ? 'bg-custom-chocolat/10 text-custom-chocolat'
              : 'bg-custom-green/10 text-custom-green'"
          >
            <font-awesome-icon
              :icon="['fas', demande.type_objet === 'animation_programme' ? 'microphone' : 'lightbulb']"
              class="w-3 h-3"
            />
            {{ demande.type_objet === 'animation_programme' ? 'Demande d\'animation' : 'Idée de contenu' }}
          </span>
          <span class="text-xs text-gray-500">{{ dateFormatee(demande.created_at) }}</span>
        </div>

        <h4 class="mt-3 font-semibold text-gray-900">{{ titre(demande) }}</h4>
        <p class="mt-1 text-sm text-gray-700">{{ demande.justification }}</p>
        <p class="mt-2 text-xs text-gray-500">Proposée par {{ auteur(demande) }}</p>

        <!-- Dire ce que l'acceptation engage : ajouter un co-détenteur n'est pas
             un simple accusé de réception, il ouvre la grille à un tiers. -->
        <p
          v-if="demande.type_objet === 'animation_programme' && peutDecider"
          class="mt-3 rounded-lg bg-amber-50 px-3 py-2 text-xs text-amber-800"
        >
          Accepter cette demande ajoutera {{ auteur(demande) }} à l'équipe du
          support : elle ou il pourra alors programmer des créneaux.
        </p>

        <div v-if="peutDecider" class="mt-4 flex flex-wrap gap-2">
          <button
            type="button"
            :disabled="traitement === demande.id"
            class="inline-flex cursor-pointer items-center gap-2 rounded-full bg-custom-green px-4 py-1.5 text-sm font-semibold text-white transition-colors hover:bg-custom-green/90 disabled:opacity-60"
            @click="accepter(demande)"
          >
            <font-awesome-icon :icon="['fas', 'check']" class="w-3.5 h-3.5" />
            Accepter
          </button>
          <button
            type="button"
            :disabled="traitement === demande.id"
            class="inline-flex cursor-pointer items-center gap-2 rounded-full border border-gray-300 px-4 py-1.5 text-sm font-semibold text-gray-700 transition-colors hover:bg-gray-50 disabled:opacity-60"
            @click="refusOuvert = refusOuvert === demande.id ? null : demande.id"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-3.5 h-3.5" />
            Refuser
          </button>
        </div>

        <p v-else class="mt-4 text-xs text-gray-500">
          Seul le propriétaire du support décide de ces demandes.
        </p>

        <div v-if="refusOuvert === demande.id" class="mt-3">
          <label class="mb-1 block text-sm font-medium text-gray-700">
            Motif du refus
            <span class="font-normal text-gray-400">(10 caractères minimum)</span>
          </label>
          <textarea
            v-model="motifs[demande.id]"
            rows="2"
            placeholder="Expliquez votre décision à l'auteur…"
            class="w-full resize-none rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-transparent focus:ring-2 focus:ring-custom-chocolat"
          ></textarea>
          <button
            type="button"
            :disabled="(motifs[demande.id] ?? '').trim().length < 10 || traitement === demande.id"
            class="mt-2 cursor-pointer rounded-full bg-gray-800 px-4 py-1.5 text-sm font-semibold text-white transition-colors hover:bg-gray-900 disabled:cursor-default disabled:opacity-50"
            @click="refuser(demande)"
          >
            Confirmer le refus
          </button>
        </div>
      </li>
    </ul>
  </div>
</template>
